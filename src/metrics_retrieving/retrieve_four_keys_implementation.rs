use async_std::stream::StreamExt;
use async_trait::async_trait;
use futures::future::try_join_all;

use crate::{
    common_types::{
        daily_items::DailyItems, date_time_range::DateTimeRange, monthly_items::MonthlyItems,
        weekly_items::WeeklyItems,
    },
    dependencies::{
        deployments_fetcher::interface::{
            BaseCommitShaOrRepositoryInfo, DeploymentLog, DeploymentsFetcher,
            DeploymentsFetcherParams,
        },
        first_commit_getter::interface::{FirstCommitGetter, ValidatedFirstCommitGetterParams},
    },
    metrics_retrieving::retrieve_four_keys_public_types::FirstCommitOrRepositoryInfo,
    shared::median::median,
};

use super::{
    retrieve_four_keys::{
        Context, DeploymentFrequency, DeploymentFrequencyLabel, DeploymentFrequencyPerformance,
        DeploymentFrequencyPerformanceSurvey2022,
    },
    retrieve_four_keys_internal_types::{
        AttachFirstOperationToDeploymentLogStep, CalculateDeploymentFrequency, CalculateLeadTime,
        CalculateLeadTimeForChangesSeconds, CreateEvents, DeploymentLogWithFirstOperation,
        ExtractItemsInPeriod, FetchDeploymentsParams, FetchDeploymentsStep,
        GetDeploymentPerformance2022, GetDeploymentPerformanceLabel, RetrieveFourKeysStep,
        ToMetricItem,
    },
    retrieve_four_keys_public_types::{
        DeploymentCommitItem, DeploymentPerformance, DeploymentPerformanceItem,
        DeploymentPerformanceLeadTimeForChanges, DeploymentPerformanceSummary, FourKeysResult,
        RepositoryInfo, RetrieveFourKeys, RetrieveFourKeysEvent, RetrieveFourKeysEventError,
        RetrieveFourKeysExecutionContext,
    },
};

// ---------------------------
// FetchDeploymentsStep
// ---------------------------
struct FetchDeploymentsStepImpl<F: DeploymentsFetcher> {
    deployments_fetcher: F,
}
#[async_trait]
impl<F: DeploymentsFetcher + Sync + Send> FetchDeploymentsStep for FetchDeploymentsStepImpl<F> {
    async fn fetch_deployments(
        self,
        params: FetchDeploymentsParams,
    ) -> Result<Vec<DeploymentLog>, RetrieveFourKeysEventError> {
        let deployments = self
            .deployments_fetcher
            .fetch(DeploymentsFetcherParams {
                timeframe: params.timeframe,
            })
            .await?;

        Ok(deployments)
    }
}

// ---------------------------
// AttachFirstOperationToDeploymentLogStep
// ---------------------------
struct AttachFirstOperationToDeploymentLogStepImpl<F: FirstCommitGetter> {
    first_commit_getter: F,
}
#[async_trait]
impl<F: FirstCommitGetter + Sync + Send> AttachFirstOperationToDeploymentLogStep
    for AttachFirstOperationToDeploymentLogStepImpl<F>
{
    async fn attach_first_operation_to_deployment_item(
        &self,
        deployment_item: DeploymentLog,
    ) -> Result<DeploymentLogWithFirstOperation, RetrieveFourKeysEventError> {
        let first_operation: Option<FirstCommitOrRepositoryInfo> =
            match deployment_item.clone().base {
                BaseCommitShaOrRepositoryInfo::BaseCommitSha(first_commit_sha) => {
                    let params = ValidatedFirstCommitGetterParams::new(
                        first_commit_sha.clone(),
                        deployment_item.clone().head_commit.sha,
                    );
                    if let Ok(params) = params {
                        let commit = self.first_commit_getter.get(params).await?;
                        Some(FirstCommitOrRepositoryInfo::FirstCommit(
                            DeploymentCommitItem {
                                sha: commit.sha,
                                message: commit.message,
                                resource_path: commit.resource_path,
                                committed_at: commit.committed_at,
                                creator_login: commit.creator_login,
                            },
                        ))
                    } else {
                        None
                    }
                }
                BaseCommitShaOrRepositoryInfo::RepositoryCreatedAt(created_at) => Some(
                    FirstCommitOrRepositoryInfo::RepositoryInfo(RepositoryInfo { created_at }),
                ),
            };
        Ok(DeploymentLogWithFirstOperation {
            deployment: deployment_item,
            first_operation,
        })
    }

    async fn attach_first_operation_to_deployment_items(
        &self,
        deployment_items: Vec<DeploymentLog>,
    ) -> Result<Vec<DeploymentLogWithFirstOperation>, RetrieveFourKeysEventError> {
        let futures = deployment_items
            .into_iter()
            .map(|it| self.attach_first_operation_to_deployment_item(it))
            .collect::<Vec<_>>();
        let results = try_join_all(futures).await?;
        let stream = futures::stream::iter(results);
        let results = stream
            .filter_map(|it| it.clone().first_operation.map(|_| it))
            .collect::<Vec<_>>()
            .await;

        Ok(results)
    }
}

// ---------------------------
// CalculationEachDeploymentsStep
// ---------------------------
pub(super) const calculate_lead_time_for_changes_seconds: CalculateLeadTimeForChangesSeconds =
    |item: DeploymentLogWithFirstOperation| -> Option<i64> {
        if let Some(operation) = item.first_operation {
            let first_committed_at = match operation {
                FirstCommitOrRepositoryInfo::FirstCommit(commit) => commit.committed_at,
                FirstCommitOrRepositoryInfo::RepositoryInfo(info) => info.created_at,
            };
            let deployed_at = item.deployment.deployed_at;
            let lead_time_for_changes_seconds = (deployed_at - first_committed_at).num_seconds();
            Some(lead_time_for_changes_seconds)
        } else {
            None
        }
    };

// NOTE: Should I write using "From"?
const to_metric_item: ToMetricItem =
    |item: DeploymentLogWithFirstOperation| -> DeploymentPerformanceItem {
        let lead_time_for_changes_seconds = calculate_lead_time_for_changes_seconds(item.clone());

        let head_commit = DeploymentCommitItem {
            sha: item.deployment.head_commit.sha,
            message: item.deployment.head_commit.message,
            resource_path: item.deployment.head_commit.resource_path,
            committed_at: item.deployment.head_commit.committed_at,
            creator_login: item.deployment.head_commit.creator_login,
        };
        let first_commit =
            item.first_operation
                .unwrap_or(FirstCommitOrRepositoryInfo::FirstCommit(
                    head_commit.clone(),
                ));
        DeploymentPerformanceItem {
            info: item.deployment.info,
            head_commit,
            first_commit,
            deployed_at: item.deployment.deployed_at,
            lead_time_for_changes_seconds,
        }
    };

// ---------------------------
// Calculation step
// ---------------------------
const extract_items_for_period: ExtractItemsInPeriod =
    |metric_items: Vec<DeploymentPerformanceItem>, timeframe: DateTimeRange| {
        metric_items
            .into_iter()
            .filter(|it| timeframe.is_include(&it.deployed_at))
            .collect::<Vec<DeploymentPerformanceItem>>()
    };

const calculate_deployment_frequency: CalculateDeploymentFrequency =
    |items: Vec<DeploymentPerformanceItem>, context: Context| {
        let total_deployments = items.len() as u32;
        let deployment_frequency_per_day = total_deployments as f32
            / (context.timeframe.num_days() as f32 * (context.working_days_per_week / 7.0));
        let deploys_per_a_day_per_a_developer =
            deployment_frequency_per_day / context.developers as f32;

        let weekly_deployment_counts = WeeklyItems::new(
            items.clone(),
            |it| it.deployed_at.date_naive(),
            context.timeframe.clone(),
        )
        .iter()
        .map(|(_week, items)| items.len() as i64)
        .collect::<Vec<_>>();
        let weekly_deployments = WeeklyItems::new(
            items.clone(),
            |it| it.deployed_at.date_naive(),
            context.timeframe.clone(),
        )
        .iter()
        .map(|(_week, items)| if items.is_empty() { 0 } else { 1 })
        .collect::<Vec<i64>>();
        let monthly_deployments =
            MonthlyItems::new(items, |it| it.deployed_at.date_naive(), context.timeframe)
                .iter()
                .map(|(_month, items)| if items.is_empty() { 0 } else { 1 })
                .collect::<Vec<i64>>();
        log::debug!("weekly_deployment_counts: {:?}", weekly_deployment_counts);
        log::debug!("weekly_deployments: {:?}", weekly_deployments);
        log::debug!("monthly_deployments: {:?}", monthly_deployments);
        let weekly_deploy_median = median(weekly_deployment_counts);
        let deployment_week_median = median(weekly_deployments);
        let deployment_month_median = median(monthly_deployments);

        DeploymentFrequency {
            total_deployments,
            weekly_deploy_median,
            deployment_week_median,
            deployment_month_median,
            deployment_frequency_per_day,
            deploys_per_a_day_per_a_developer,
        }
    };

const get_deployment_performance2022: GetDeploymentPerformance2022 =
    |_deployment_frequency: DeploymentFrequency,
     label: DeploymentFrequencyLabel,
     _context|
     -> DeploymentFrequencyPerformanceSurvey2022 {
        match label {
            DeploymentFrequencyLabel::Daily => DeploymentFrequencyPerformanceSurvey2022::High,
            DeploymentFrequencyLabel::Weekly => DeploymentFrequencyPerformanceSurvey2022::Medium,
            _ => DeploymentFrequencyPerformanceSurvey2022::Low,
        }
    };

const get_deployment_performance_label: GetDeploymentPerformanceLabel =
    |deployment_frequency: DeploymentFrequency, context| -> DeploymentFrequencyLabel {
        let coefficient = context.working_days_per_week as f64 * (3.0 / 5.0);
        if deployment_frequency.weekly_deploy_median > coefficient {
            DeploymentFrequencyLabel::Daily
        } else if deployment_frequency.deployment_week_median >= 1.0 {
            DeploymentFrequencyLabel::Weekly
        } else if deployment_frequency.deployment_month_median >= 1.0 {
            DeploymentFrequencyLabel::Monthly
        } else {
            DeploymentFrequencyLabel::Yearly
        }
    };

const calculate_lead_time: CalculateLeadTime =
    |items: Vec<DeploymentPerformanceItem>| -> DeploymentPerformanceLeadTimeForChanges {
        let durations = items
            .iter()
            .flat_map(|item| item.lead_time_for_changes_seconds)
            .collect::<Vec<i64>>();
        log::debug!("durations: {:?}", durations);
        let median_duration = median(durations);
        let days = (median_duration / 86400.0) as i64;
        let hours = (median_duration / 3600.0) as i64;
        let minutes = (median_duration.round() as i64 % 3600) / 60;
        let seconds = (median_duration.round() as i64) - (hours * 3600) - (minutes * 60);
        DeploymentPerformanceLeadTimeForChanges {
            days,
            hours,
            minutes,
            seconds,
            total_seconds: median_duration,
        }
    };

// ---------------------------
// Retrieve FourKeys event
// ---------------------------
struct RetrieveFourKeysStepImpl<
    FDeploymentsFetcher: DeploymentsFetcher,
    FFirstCommitGetter: FirstCommitGetter,
> {
    pub deployments_fetcher: FDeploymentsFetcher,
    pub first_commit_getter: FFirstCommitGetter,
}
#[async_trait]
impl<
        FDeploymentsFetcher: DeploymentsFetcher + Sync + Send,
        FFirstCommitGetter: FirstCommitGetter + Sync + Send,
    > RetrieveFourKeysStep for RetrieveFourKeysStepImpl<FDeploymentsFetcher, FFirstCommitGetter>
{
    async fn retrieve_four_keys(
        self,
        context: RetrieveFourKeysExecutionContext,
    ) -> Result<FourKeysResult, RetrieveFourKeysEventError> {
        let context = Context {
            timeframe: context.timeframe,
            developers: context.project.developer_count,
            working_days_per_week: context.project.working_days_per_week,
        };
        let fetch_deployments_step = FetchDeploymentsStepImpl {
            deployments_fetcher: self.deployments_fetcher,
        };
        let deployments = fetch_deployments_step
            .fetch_deployments(FetchDeploymentsParams {
                timeframe: context.timeframe.clone(),
            })
            .await?;
        let deployments_with_first_operation = AttachFirstOperationToDeploymentLogStepImpl {
            first_commit_getter: self.first_commit_getter,
        }
        .attach_first_operation_to_deployment_items(deployments)
        .await?;
        let metrics_items: Vec<DeploymentPerformanceItem> = deployments_with_first_operation
            .into_iter()
            .map(to_metric_item)
            .collect();
        let mut sorted_items = metrics_items;
        sorted_items.sort_by_key(|item| item.deployed_at);
        let extracted_items = extract_items_for_period(sorted_items, context.timeframe.clone());

        let deployment_frequency_value =
            calculate_deployment_frequency(extracted_items.clone(), context.clone());
        let label =
            get_deployment_performance_label(deployment_frequency_value.clone(), context.clone());
        let deployment_frequency = DeploymentFrequencyPerformance {
            label: label.clone(),
            value: deployment_frequency_value.clone(),
            performance: get_deployment_performance2022(
                deployment_frequency_value,
                label,
                context.clone(),
            ),
        };

        let lead_time_for_changes = calculate_lead_time(extracted_items.clone());

        let performance = DeploymentPerformance {
            deployment_frequency,
            lead_time_for_changes,
        };

        let deployment_frequencies_by_date = DailyItems::new(
            extracted_items,
            |item| item.deployed_at.date_naive(),
            context.timeframe.clone(),
        )
        .iter()
        .map(|(date, daily_items)| DeploymentPerformanceSummary {
            date: *date,
            deploys: daily_items.len() as u32,
            items: daily_items.to_vec(),
        })
        .collect::<Vec<DeploymentPerformanceSummary>>();
        let mut sorted_deployment_frequencies_by_date = deployment_frequencies_by_date;
        sorted_deployment_frequencies_by_date.sort_by_key(|item| item.date);

        let deployment_frequency = FourKeysResult {
            deployments: sorted_deployment_frequencies_by_date,
            context,
            performance,
        };

        Ok(deployment_frequency)
    }
}

// ---------------------------
// create events
// ---------------------------
const create_events: CreateEvents = |project: FourKeysResult| -> Vec<RetrieveFourKeysEvent> {
    vec![RetrieveFourKeysEvent::RetrieveFourKeys(project)]
};

// ---------------------------
// overall workflow
// ---------------------------
pub struct RetrieveFourKeysWorkflow<
    FDeploymentsFetcher: DeploymentsFetcher,
    FFirstCommitGetter: FirstCommitGetter,
> {
    pub deployments_fetcher: FDeploymentsFetcher,
    pub first_commit_getter: FFirstCommitGetter,
}
#[async_trait]
impl<
        FDeploymentsFetcher: DeploymentsFetcher + Sync + Send,
        FFirstCommitGetter: FirstCommitGetter + Sync + Send,
    > RetrieveFourKeys for RetrieveFourKeysWorkflow<FDeploymentsFetcher, FFirstCommitGetter>
{
    async fn retrieve_four_keys(
        self,
        context: RetrieveFourKeysExecutionContext,
    ) -> Result<Vec<RetrieveFourKeysEvent>, RetrieveFourKeysEventError> {
        let events = create_events(
            RetrieveFourKeysStepImpl {
                deployments_fetcher: self.deployments_fetcher,
                first_commit_getter: self.first_commit_getter,
            }
            .retrieve_four_keys(context)
            .await?,
        );

        Ok(events)
    }
}
