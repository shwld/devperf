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
        AttachFirstOperationToDeploymentItemStep, ConvertToMetricItemStep,
        DeploymentItemWithFirstOperation, FetchDeploymentsParams, FetchDeploymentsStep,
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
// ConvertToMetricItemStep
// ---------------------------
struct ConvertToMetricItemStepImpl {}
impl ConvertToMetricItemStep for ConvertToMetricItemStepImpl {
    fn calculate_lead_time_for_changes_seconds(
        &self,
        item: DeploymentItemWithFirstOperation,
    ) -> Option<i64> {
        if let Some(operation) = item.first_operation {
            let first_committed_at = match operation {
                FirstCommitOrRepositoryInfo::FirstCommit(commit) => commit.committed_at,
                FirstCommitOrRepositoryInfo::RepositoryInfo(info) => info.created_at,
            };
            Some((item.deployment.deployed_at - first_committed_at).num_seconds())
        } else {
            None
        }
    }

    fn to_metric_item(&self, item: DeploymentItemWithFirstOperation) -> DeploymentMetricItem {
        let lead_time_for_changes_seconds =
            self.calculate_lead_time_for_changes_seconds(item.clone());

        let head_commit = item.deployment.head_commit.clone();
        let first_commit =
            item.first_operation
                .unwrap_or(FirstCommitOrRepositoryInfo::FirstCommit(
                    DeploymentCommitItem {
                        sha: item.deployment.head_commit.sha,
                        message: item.deployment.head_commit.message,
                        resource_path: item.deployment.head_commit.resource_path,
                        committed_at: item.deployment.head_commit.committed_at,
                        creator_login: item.deployment.head_commit.creator_login,
                    },
                ));
        let deployment_metric = DeploymentMetricItem {
            id: item.deployment.id,
            head_commit: DeploymentCommitItem {
                sha: head_commit.sha,
                message: head_commit.message,
                resource_path: head_commit.resource_path,
                committed_at: head_commit.committed_at,
                creator_login: head_commit.creator_login,
            },
            first_commit,
            deployed_at: item.deployment.deployed_at,
            lead_time_for_changes_seconds,
        };

        deployment_metric
    }
}

// ---------------------------
// Calculation step
// ---------------------------
fn split_by_day(metrics_items: Vec<DeploymentMetricItem>) -> Vec<Vec<DeploymentMetricItem>> {
    let mut items_by_day: Vec<Vec<DeploymentMetricItem>> = Vec::new();
    let mut inner_items: Vec<DeploymentMetricItem> = Vec::new();
    let mut current_date: Option<NaiveDate> = None;

    for item in metrics_items {
        let target_time = item.deployed_at.date_naive();
        if let Some(current_time) = current_date {
            if target_time != current_time {
                current_date = Some(target_time);
                items_by_day.push(inner_items);
                inner_items = Vec::new();
            }
        }

        inner_items.push(item);
    }
    if !inner_items.is_empty() {
        items_by_day.push(inner_items);
    }

    items_by_day
}

fn calculate_four_keys(
    metrics_items: Vec<DeploymentMetricItem>,
    context: RetrieveFourKeysExecutionContext,
) -> Result<FourKeysResult, RetrieveFourKeysEventError> {
    let ranged_items = metrics_items
        .into_iter()
        .filter(|it| it.deployed_at >= context.since && it.deployed_at <= context.until)
        .collect::<Vec<DeploymentMetricItem>>();
    let items_by_day = split_by_day(ranged_items);
    log::debug!("items_by_day: {:?}", items_by_day);

    let total_deployments = items_by_day
        .iter()
        .fold(0, |total, i| total + i.len() as u32);
    let duration_since = context.until.signed_duration_since(context.since);
    let days = duration_since.num_days();
    let deployment_frequency_per_day =
        total_deployments as f32 / (days as f32 * (context.project.working_days_per_week / 7.0));

    let durations = items_by_day
        .iter()
        .flat_map(|items| items.iter())
        .flat_map(|item| item.lead_time_for_changes_seconds)
        .collect::<Vec<i64>>();
    log::debug!("durations: {:?}", durations);
    let median_duration = median(durations);
    let hours = (median_duration / 3600.0) as i64;
    let minutes = (median_duration.round() as i64 % 3600) / 60;
    let seconds = (median_duration.round() as i64) - (hours * 3600) - (minutes * 60);
    let lead_time = DeploymentMetricLeadTimeForChanges {
        hours,
        minutes,
        seconds,
        total_seconds: median_duration,
    };

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

    let deployment_frequencies_by_day = items_by_day
        .into_iter()
        .map(|items| {
            // TODO: 型定義でTotalityを確保したい
            let date = items[0].deployed_at.date_naive();
            let deployments = items.len() as u32;
            DeploymentMetricSummary {
                date,
                deploys: deployments,
                items,
            }
        })
        .collect::<Vec<DeploymentMetricSummary>>();

    let deployment_frequency = FourKeysResult {
        metrics,
        deployments: deployment_frequencies_by_day,
    };

    Ok(deployment_frequency)
}

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
        .map(|it| ConvertToMetricItemStepImpl {}.to_metric_item(it))
        .collect::<Vec<DeploymentMetricItem>>();
    let result = calculate_four_keys(metrics_items, context.clone())?;

    Ok(result)
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
