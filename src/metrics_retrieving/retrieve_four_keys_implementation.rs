use async_trait::async_trait;
use futures::future::try_join_all;

use super::{
    retrieve_four_keys::{
        Context, DeploymentFrequency, DeploymentFrequencyLabel, DeploymentFrequencyPerformance,
        DeploymentFrequencyPerformanceSurvey2022,
    },
    retrieve_four_keys_internal_types::{
        CalculateDeploymentFrequency, CalculateLeadTime, CalculateLeadTimeMedian, CreateEvents,
        DeploymentLogWithFirstOperation, GetDeploymentPerformance2022,
        GetDeploymentPerformanceLabel, PickFirstCommit, RetrieveFourKeysStep,
    },
    retrieve_four_keys_public_types::{
        DailyDeploymentsSummary, Deployment, DeploymentLeadTimeForChanges, DeploymentPerformance,
        FourKeysResult, RepositoryInfo, RetrieveFourKeys, RetrieveFourKeysEvent,
        RetrieveFourKeysEventError, RetrieveFourKeysExecutionContext,
    },
};
use crate::{
    common_types::{
        commit::Commit, daily_items::DailyItems, monthly_items::MonthlyItems,
        weekly_items::WeeklyItems,
    },
    dependencies::{
        deployments_fetcher::interface::{
            BaseCommitShaOrRepositoryInfo, DeploymentsFetcher, DeploymentsFetcherParams,
        },
        two_commits_comparer::interface::{TwoCommitsComparer, ValidatedCommitShaPair},
    },
    metrics_retrieving::retrieve_four_keys_public_types::FirstCommitOrRepositoryInfo,
    shared::median::median,
};

// ---------------------------
// PickFirstCommit
// ---------------------------
const pick_first_commit: PickFirstCommit = |commits: &Vec<Commit>| -> Option<Commit> {
    let mut sorted_commits = commits.clone();
    sorted_commits.sort_by_key(|it| it.committed_at);
    sorted_commits.first().cloned()
};

// ---------------------------
// CalculateEachLogLeadTimes
// ---------------------------
pub(super) fn calculate_lead_time_for_changes_seconds(
    log_with_operation: DeploymentLogWithFirstOperation,
) -> Option<i64> {
    if let Some(operation) = log_with_operation.first_operation {
        let first_committed_at = match operation {
            FirstCommitOrRepositoryInfo::FirstCommit(commit) => commit.committed_at,
            FirstCommitOrRepositoryInfo::RepositoryInfo(info) => info.created_at,
        };
        let deployed_at = log_with_operation.deployment_log.deployed_at;
        let lead_time_for_changes_seconds = (deployed_at - first_committed_at).num_seconds();
        Some(lead_time_for_changes_seconds)
    } else {
        None
    }
}

const calculate_lead_time: CalculateLeadTime =
    |log_with_operation: DeploymentLogWithFirstOperation| -> Deployment {
        let lead_time_for_changes_seconds =
            calculate_lead_time_for_changes_seconds(log_with_operation.clone());

        let head_commit = Commit {
            sha: log_with_operation.deployment_log.head_commit.sha,
            message: log_with_operation.deployment_log.head_commit.message,
            resource_path: log_with_operation.deployment_log.head_commit.resource_path,
            committed_at: log_with_operation.deployment_log.head_commit.committed_at,
            creator_login: log_with_operation.deployment_log.head_commit.creator_login,
        };
        let first_commit =
            log_with_operation
                .first_operation
                .unwrap_or(FirstCommitOrRepositoryInfo::FirstCommit(
                    head_commit.clone(),
                ));
        Deployment {
            info: log_with_operation.deployment_log.info,
            head_commit,
            first_commit,
            deployed_at: log_with_operation.deployment_log.deployed_at,
            lead_time_for_changes_seconds,
        }
    };

// ---------------------------
// Aggregation
// ---------------------------
const calculate_deployment_frequency: CalculateDeploymentFrequency =
    |items: Vec<Deployment>, context: &Context| {
        let total_deployments = items.len() as u32;
        let deployment_days: i32 = DailyItems::new(
            items.clone(),
            |item| item.deployed_at.date_naive(),
            context.timeframe.clone(),
        )
        .nonempty_items()
        .count() as i32;
        let timeframe_days = context.timeframe.num_days() as f32;
        let deployment_frequency_per_day =
            deployment_days as f32 / (timeframe_days * (context.working_days_per_week / 7.0));
        let deploys_per_a_day_per_a_developer =
            deployment_frequency_per_day / context.developers as f32;

        let weekly_deployments = WeeklyItems::new(
            items.clone(),
            |it| it.deployed_at.date_naive(),
            context.timeframe.clone(),
        );
        let weekly_deployment_counts = weekly_deployments
            .iter()
            .map(|(_week, items)| items.len() as i64)
            .collect::<Vec<_>>();
        let weekly_deployments = weekly_deployments
            .iter()
            .map(|(_week, items)| if items.is_empty() { 0 } else { 1 })
            .collect::<Vec<i64>>();
        let monthly_deployments =
            MonthlyItems::new(items, |it| it.deployed_at.date_naive(), &context.timeframe)
                .iter()
                .map(|(_month, items)| if items.is_empty() { 0 } else { 1 })
                .collect::<Vec<i64>>();
        log::debug!("weekly_deployment_counts: {:?}", weekly_deployment_counts);
        log::debug!("weekly_deployments: {:?}", weekly_deployments);
        log::debug!("monthly_deployments: {:?}", monthly_deployments);
        let weekly_deployment_count_median = median(weekly_deployment_counts);
        let week_deployed_median = median(weekly_deployments);
        let month_deployed_median = median(monthly_deployments);

        DeploymentFrequency {
            total_deployments,
            weekly_deployment_count_median,
            week_deployed_median,
            month_deployed_median,
            deployment_frequency_per_day,
            deploys_per_a_day_per_a_developer,
        }
    };

const get_deployment_performance2022: GetDeploymentPerformance2022 =
    |_deployment_frequency: &DeploymentFrequency,
     label: &DeploymentFrequencyLabel,
     _context: &Context|
     -> DeploymentFrequencyPerformanceSurvey2022 {
        match label {
            DeploymentFrequencyLabel::Daily => DeploymentFrequencyPerformanceSurvey2022::High,
            DeploymentFrequencyLabel::Weekly => DeploymentFrequencyPerformanceSurvey2022::Medium,
            _ => DeploymentFrequencyPerformanceSurvey2022::Low,
        }
    };

const get_deployment_performance_label: GetDeploymentPerformanceLabel =
    |deployment_frequency: &DeploymentFrequency, context: &Context| -> DeploymentFrequencyLabel {
        let coefficient = context.working_days_per_week as f64 * (3.0 / 5.0);
        if deployment_frequency.weekly_deployment_count_median > coefficient {
            DeploymentFrequencyLabel::Daily
        } else if deployment_frequency.week_deployed_median >= 1.0 {
            DeploymentFrequencyLabel::Weekly
        } else if deployment_frequency.month_deployed_median >= 1.0 {
            DeploymentFrequencyLabel::Monthly
        } else {
            DeploymentFrequencyLabel::Yearly
        }
    };

const calculate_lead_time_median: CalculateLeadTimeMedian =
    |items: &Vec<Deployment>| -> DeploymentLeadTimeForChanges {
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
        DeploymentLeadTimeForChanges {
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
    FTwoCommitsComparer: TwoCommitsComparer,
> {
    pub deployments_fetcher: FDeploymentsFetcher,
    pub two_commits_comparer: FTwoCommitsComparer,
}
#[async_trait]
impl<
        FDeploymentsFetcher: DeploymentsFetcher + Sync + Send,
        FTwoCommitsComparer: TwoCommitsComparer + Sync + Send,
    > RetrieveFourKeysStep for RetrieveFourKeysStepImpl<FDeploymentsFetcher, FTwoCommitsComparer>
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
        let deployment_logs = self
            .deployments_fetcher
            .fetch(DeploymentsFetcherParams {
                timeframe: context.timeframe.clone(),
            })
            .await?;
        let deployment_logs = deployment_logs
            .into_iter()
            .filter(|log| context.timeframe.is_include(&log.deployed_at))
            .collect::<Vec<_>>();
        let deployment_with_first_operations =
            try_join_all(deployment_logs.iter().map(|log| async {
                let first_operation = match log.base.clone() {
                    BaseCommitShaOrRepositoryInfo::BaseCommitSha(sha) => {
                        let commit_sha_pair =
                            ValidatedCommitShaPair::new(sha.clone(), log.head_commit.sha.clone());
                        match commit_sha_pair {
                            Ok(commit_sha_pair) => {
                                let commits =
                                    self.two_commits_comparer.compare(commit_sha_pair).await?;
                                let first_commit = pick_first_commit(&commits);

                                Ok(first_commit.map(FirstCommitOrRepositoryInfo::FirstCommit))
                            }
                            Err(e) => Err(RetrieveFourKeysEventError::InvalidCommitShaPair(e)),
                        }
                    }
                    BaseCommitShaOrRepositoryInfo::RepositoryCreatedAt(created_at) => Ok(Some(
                        FirstCommitOrRepositoryInfo::RepositoryInfo(RepositoryInfo { created_at }),
                    )),
                };
                match first_operation {
                    Ok(first_operation) => Ok(DeploymentLogWithFirstOperation {
                        deployment_log: log.clone(),
                        first_operation,
                    }),
                    Err(e) => Err(e),
                }
            }))
            .await?;
        let deployments: Vec<Deployment> = deployment_with_first_operations
            .into_iter()
            .map(calculate_lead_time)
            .collect();
        let mut sorted_deployments = deployments;
        sorted_deployments.sort_by_key(|item| item.deployed_at);

        let deployment_frequency_value =
            calculate_deployment_frequency(sorted_deployments.clone(), &context);
        let label = get_deployment_performance_label(&deployment_frequency_value, &context);
        let performance =
            get_deployment_performance2022(&deployment_frequency_value, &label, &context);
        let deployment_frequency = DeploymentFrequencyPerformance {
            label,
            value: deployment_frequency_value,
            performance,
        };

        let lead_time_for_changes = calculate_lead_time_median(&sorted_deployments);

        let performance = DeploymentPerformance {
            deployment_frequency,
            lead_time_for_changes,
        };

        let daily_deployment_summaries: Vec<DailyDeploymentsSummary> = DailyItems::new(
            sorted_deployments,
            |item| item.deployed_at.date_naive(),
            context.timeframe.clone(),
        )
        .iter()
        .map(|(date, daily_items)| DailyDeploymentsSummary {
            date: *date,
            deploys: daily_items.len() as u32,
            items: daily_items.to_vec(),
        })
        .collect();
        let mut sorted_daily_deployment_summaries = daily_deployment_summaries;
        sorted_daily_deployment_summaries.sort_by_key(|item| item.date);

        let deployment_frequency = FourKeysResult {
            deployments: sorted_daily_deployment_summaries,
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
    FTwoCommitsComparer: TwoCommitsComparer,
> {
    pub deployments_fetcher: FDeploymentsFetcher,
    pub two_commits_comparer: FTwoCommitsComparer,
}
#[async_trait]
impl<
        FDeploymentsFetcher: DeploymentsFetcher + Sync + Send,
        FTwoCommitsComparer: TwoCommitsComparer + Sync + Send,
    > RetrieveFourKeys for RetrieveFourKeysWorkflow<FDeploymentsFetcher, FTwoCommitsComparer>
{
    async fn retrieve_four_keys(
        self,
        context: RetrieveFourKeysExecutionContext,
    ) -> Result<Vec<RetrieveFourKeysEvent>, RetrieveFourKeysEventError> {
        let events = create_events(
            RetrieveFourKeysStepImpl {
                deployments_fetcher: self.deployments_fetcher,
                two_commits_comparer: self.two_commits_comparer,
            }
            .retrieve_four_keys(context)
            .await?,
        );

        Ok(events)
    }
}
