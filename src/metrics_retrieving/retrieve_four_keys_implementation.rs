use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use futures::future::try_join_all;

use crate::{
    dependencies::{
        deployments_fetcher::interface::{
            CommitOrRepositoryInfo, DeploymentItem, DeploymentsFetcher, DeploymentsFetcherParams,
        },
        first_commit_getter::interface::{FirstCommitGetter, ValidatedFirstCommitGetterParams},
    },
    metrics_retrieving::retrieve_four_keys_public_types::FirstCommitOrRepositoryInfo,
    shared::median::median,
};

use super::{
    retrieve_four_keys_internal_types::{
        AttachFirstOperationToDeploymentItemStep, CalculateDeploymentFrequencyPerDay,
        CalculateLeadTime, CalculateLeadTimeForChangesSeconds, CalculateTotalDeployments,
        DailyItems, DeploymentItemWithFirstOperation, ExtractItemsInPeriod, FetchDeploymentsParams,
        FetchDeploymentsStep, GroupByDate, ToMetricItem,
    },
    retrieve_four_keys_public_types::{
        DeploymentCommitItem, DeploymentMetric, DeploymentMetricItem,
        DeploymentMetricLeadTimeForChanges, DeploymentMetricSummary, FourKeysResult,
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
    ) -> Result<Vec<DeploymentItem>, RetrieveFourKeysEventError> {
        let deployments = self
            .deployments_fetcher
            .fetch(DeploymentsFetcherParams {
                since: Some(params.since),
            })
            .await?;

        Ok(deployments)
    }
}

// ---------------------------
// AttachFirstOperationToDeploymentItemStep
// ---------------------------
struct AttachFirstOperationToDeploymentItemStepImpl<F: FirstCommitGetter> {
    first_commit_getter: F,
}
#[async_trait]
impl<F: FirstCommitGetter + Sync + Send> AttachFirstOperationToDeploymentItemStep
    for AttachFirstOperationToDeploymentItemStepImpl<F>
{
    async fn attach_first_operation_to_deployment_item(
        &self,
        deployment_item: DeploymentItem,
    ) -> Result<DeploymentItemWithFirstOperation, RetrieveFourKeysEventError> {
        let first_operation: Option<FirstCommitOrRepositoryInfo> =
            match deployment_item.clone().base {
                CommitOrRepositoryInfo::Commit(first_commit) => {
                    let params = ValidatedFirstCommitGetterParams::new(
                        first_commit.sha.clone(),
                        deployment_item.clone().head_commit.sha.clone(),
                    );
                    let commit = if let Ok(params) = params {
                        let commit = self.first_commit_getter.get(params).await?;
                        log::debug!("first_commit: {:?}", commit);
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
                    };
                    commit
                }
                CommitOrRepositoryInfo::RepositoryInfo(info) => Some(
                    FirstCommitOrRepositoryInfo::RepositoryInfo(RepositoryInfo {
                        created_at: info.created_at,
                    }),
                ),
            };
        Ok(DeploymentItemWithFirstOperation {
            deployment: deployment_item,
            first_operation,
        })
    }

    async fn attach_first_operation_to_deployment_items(
        &self,
        deployment_items: Vec<DeploymentItem>,
    ) -> Result<Vec<DeploymentItemWithFirstOperation>, RetrieveFourKeysEventError> {
        let futures = deployment_items
            .into_iter()
            .map(|it| self.attach_first_operation_to_deployment_item(it))
            .collect::<Vec<_>>();
        let results = try_join_all(futures).await?;
        Ok(results)
    }
}

// ---------------------------
// CalculationEachDeploymentsStep
// ---------------------------
const calculate_lead_time_for_changes_seconds: CalculateLeadTimeForChangesSeconds =
    |item: DeploymentItemWithFirstOperation| -> Option<i64> {
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
    |item: DeploymentItemWithFirstOperation| -> DeploymentMetricItem {
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
        let deployment_metric = DeploymentMetricItem {
            id: item.deployment.id,
            head_commit,
            first_commit,
            deployed_at: item.deployment.deployed_at,
            lead_time_for_changes_seconds,
        };

        deployment_metric
    };

// ---------------------------
// Calculation step
// ---------------------------
const group_by_date: GroupByDate = |metrics_items: Vec<DeploymentMetricItem>| -> Vec<DailyItems> {
    let mut items_by_date: Vec<DailyItems> = Vec::new();
    let mut inner_items = Vec::new();
    let mut current_date: Option<NaiveDate> = None;

    for item in metrics_items {
        let target_date = item.deployed_at.date_naive();
        if let Some(current_time) = current_date {
            if target_date != current_time {
                current_date = Some(target_date);
                items_by_date.push(DailyItems {
                    date: target_date,
                    items: inner_items,
                });
                inner_items = Vec::new();
            }
        }

        inner_items.push(item);
    }
    if !inner_items.is_empty() {
        if let Some(current_date) = current_date {
            items_by_date.push(DailyItems {
                date: current_date,
                items: inner_items,
            });
        }
    }

    items_by_date
};

const extract_items_for_period: ExtractItemsInPeriod =
    |metric_items: Vec<DeploymentMetricItem>, since: DateTime<Utc>, until: DateTime<Utc>| {
        metric_items
            .into_iter()
            .filter(|it| it.deployed_at >= since && it.deployed_at <= until)
            .collect::<Vec<DeploymentMetricItem>>()
    };

const calculate_total_deployments: CalculateTotalDeployments = |items: Vec<DailyItems>| -> u32 {
    items
        .iter()
        .fold(0, |total, i| total + i.items.len() as u32)
};

const calculate_deployment_frequency_per_day: CalculateDeploymentFrequencyPerDay =
    |total_deployments: u32,
     since: DateTime<Utc>,
     until: DateTime<Utc>,
     working_days_per_week: f32| {
        let duration_since = until.signed_duration_since(since);
        let days = duration_since.num_days();
        total_deployments as f32 / (days as f32 * (working_days_per_week / 7.0))
    };

const calculate_lead_time: CalculateLeadTime =
    |items: Vec<DailyItems>| -> DeploymentMetricLeadTimeForChanges {
        let durations = items
            .iter()
            .flat_map(|daily| daily.items.iter())
            .flat_map(|item| item.lead_time_for_changes_seconds)
            .collect::<Vec<i64>>();
        log::debug!("durations: {:?}", durations);
        let median_duration = median(durations);
        let hours = (median_duration / 3600.0) as i64;
        let minutes = (median_duration.round() as i64 % 3600) / 60;
        let seconds = (median_duration.round() as i64) - (hours * 3600) - (minutes * 60);
        DeploymentMetricLeadTimeForChanges {
            hours,
            minutes,
            seconds,
            total_seconds: median_duration,
        }
    };

// ---------------------------
// Retrieve FourKeys event
// ---------------------------
async fn retrieve_four_keys<
    FDeploymentsFetcher: DeploymentsFetcher + Sync + Send,
    FFirstCommitGetter: FirstCommitGetter + Sync + Send,
>(
    deployments_fetcher: FDeploymentsFetcher,
    first_commit_getter: FFirstCommitGetter,
    context: RetrieveFourKeysExecutionContext,
) -> Result<FourKeysResult, RetrieveFourKeysEventError> {
    let fetch_deployments_step = FetchDeploymentsStepImpl {
        deployments_fetcher,
    };
    let deployments = fetch_deployments_step
        .fetch_deployments(FetchDeploymentsParams {
            since: context.since,
            until: context.until,
        })
        .await?;
    let deployments_with_first_operation = AttachFirstOperationToDeploymentItemStepImpl {
        first_commit_getter,
    }
    .attach_first_operation_to_deployment_items(deployments)
    .await?;
    let metrics_items = deployments_with_first_operation
        .into_iter()
        .map(to_metric_item)
        .collect();
    let daily_items = group_by_date(extract_items_for_period(
        metrics_items,
        context.since,
        context.until,
    ));
    log::debug!("daily_items: {:?}", daily_items);

    let total_deployments = calculate_total_deployments(daily_items.clone());
    let deployment_frequency_per_day = calculate_deployment_frequency_per_day(
        total_deployments,
        context.since,
        context.until,
        context.project.working_days_per_week,
    );
    let lead_time = calculate_lead_time(daily_items.clone());

    let metrics = DeploymentMetric {
        since: context.since,
        until: context.until,
        developers: context.project.developer_count,
        working_days_per_week: context.project.working_days_per_week,
        deploys: total_deployments,
        deployment_frequency_per_day,
        deploys_per_a_day_per_a_developer: deployment_frequency_per_day
            / context.project.developer_count as f32,
        lead_time_for_changes: lead_time,
    };

    let deployment_frequencies_by_date = daily_items
        .into_iter()
        .map(|daily| DeploymentMetricSummary {
            date: daily.date,
            deploys: daily.items.len() as u32,
            items: daily.items,
        })
        .collect::<Vec<DeploymentMetricSummary>>();

    let deployment_frequency = FourKeysResult {
        metrics,
        deployments: deployment_frequencies_by_date,
    };

    Ok(deployment_frequency)
}

// ---------------------------
// create events
// ---------------------------
fn create_events(project: FourKeysResult) -> Vec<RetrieveFourKeysEvent> {
    vec![RetrieveFourKeysEvent::RetrieveFourKeys(project)]
}

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
            retrieve_four_keys(self.deployments_fetcher, self.first_commit_getter, context).await?,
        );

        Ok(events)
    }
}
