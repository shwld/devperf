use chrono::{DateTime, NaiveDate, Utc};
use futures::future::try_join_all;

use crate::{
    dependencies::{
        fetch_deployments::interface::{
            CommitOrRepositoryInfo, DeploymentItem, FetchDeployments, FetchDeploymentsParams,
        },
        get_first_commit_from_compare::interface::{
            FirstCommitFromCompareParams, GetFirstCommitFromCompare,
        },
        read_project_config::interface::{ProjectConfig, ResourceConfig},
    },
    metrics_retrieving::retrieve_four_keys_schema::FirstCommitOrRepositoryInfo,
};

use super::retrieve_four_keys_schema::{
    DeploymentCommitItem, DeploymentMetric, DeploymentMetricItem,
    DeploymentMetricLeadTimeForChanges, DeploymentMetricSummary, FourKeysMetrics, RepositoryInfo,
    RetrieveFourKeysEvent, RetrieveFourKeysEventError, RetrieveFourKeysExecutionContext,
};

// ---------------------------
// Fetch deployments step
// ---------------------------

async fn fetch_deployments<FGD: FetchDeployments, FHR: FetchDeployments>(
    fetch_deployments_with_github_deployments: &FGD,
    fetch_deployments_with_heroku_release: &FHR,
    project_config: ProjectConfig,
    since: DateTime<Utc>,
    environment: &str,
) -> Result<Vec<DeploymentItem>, RetrieveFourKeysEventError> {
    let deployments = match project_config.resource {
        ResourceConfig::GitHubDeployment(resource_config) => {
            fetch_deployments_with_github_deployments
                .perform(FetchDeploymentsParams {
                    owner: resource_config.github_owner,
                    repo: resource_config.github_repo,
                    environment: environment.to_string(),
                    since: Some(since),
                })
                .await
                .map_err(RetrieveFourKeysEventError::FetchDeploymentsError)
        }
        ResourceConfig::HerokuRelease(resource_config) => fetch_deployments_with_heroku_release
            .perform(FetchDeploymentsParams {
                owner: resource_config.github_owner,
                repo: resource_config.github_repo,
                environment: environment.to_string(),
                since: Some(since),
            })
            .await
            .map_err(RetrieveFourKeysEventError::FetchDeploymentsError),
    }?;

    Ok(deployments)
}

// ---------------------------
// Convert to MetricItem step
// ---------------------------

pub async fn to_metric_item<F: GetFirstCommitFromCompare>(
    get_first_commit_from_compare: &F,
    deployment: DeploymentItem,
    project_config: ProjectConfig,
) -> Result<DeploymentMetricItem, RetrieveFourKeysEventError> {
    let first_commit: Option<FirstCommitOrRepositoryInfo> = match deployment.base {
        CommitOrRepositoryInfo::Commit(first_commit) => {
            let commit = get_first_commit_from_compare
                .perform(match project_config.resource {
                    ResourceConfig::GitHubDeployment(resource_config) => {
                        FirstCommitFromCompareParams {
                            owner: resource_config.github_owner,
                            repo: resource_config.github_repo,
                            base: first_commit.sha,
                            head: deployment.head_commit.sha.clone(),
                        }
                    }
                    ResourceConfig::HerokuRelease(resource_config) => {
                        FirstCommitFromCompareParams {
                            owner: resource_config.github_owner,
                            repo: resource_config.github_repo,
                            base: first_commit.sha,
                            head: deployment.head_commit.sha.clone(),
                        }
                    }
                })
                .await;
            log::debug!("first_commit: {:?}", commit);
            match commit {
                Ok(commit) => Some(FirstCommitOrRepositoryInfo::FirstCommit(
                    DeploymentCommitItem {
                        sha: commit.sha,
                        message: commit.message,
                        resource_path: commit.resource_path,
                        committed_at: commit.committed_at,
                        creator_login: commit.creator_login,
                    },
                )),
                Err(_) => None,
            }
        }
        CommitOrRepositoryInfo::RepositoryInfo(info) => Some(
            FirstCommitOrRepositoryInfo::RepositoryInfo(RepositoryInfo {
                created_at: info.created_at,
            }),
        ),
    };
    let lead_time_for_changes_seconds = if let Some(first_commit) = first_commit.clone() {
        let first_committed_at = match first_commit {
            FirstCommitOrRepositoryInfo::FirstCommit(commit) => commit.committed_at,
            FirstCommitOrRepositoryInfo::RepositoryInfo(info) => info.created_at,
        };
        Some((deployment.deployed_at - first_committed_at).num_seconds())
    } else {
        None
    };

    let head_commit = deployment.head_commit.clone();
    let first_commit = first_commit.unwrap_or(FirstCommitOrRepositoryInfo::FirstCommit(
        DeploymentCommitItem {
            sha: deployment.head_commit.sha,
            message: deployment.head_commit.message,
            resource_path: deployment.head_commit.resource_path,
            committed_at: deployment.head_commit.committed_at,
            creator_login: deployment.head_commit.creator_login,
        },
    ));
    let deployment_metric = DeploymentMetricItem {
        id: deployment.id,
        head_commit: DeploymentCommitItem {
            sha: head_commit.sha,
            message: head_commit.message,
            resource_path: head_commit.resource_path,
            committed_at: head_commit.committed_at,
            creator_login: head_commit.creator_login,
        },
        first_commit,
        deployed_at: deployment.deployed_at,
        lead_time_for_changes_seconds,
    };

    Ok(deployment_metric)
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

fn median(numbers: Vec<i64>) -> f64 {
    let mut sorted = numbers;
    sorted.sort();

    let n = sorted.len();
    if n == 0 {
        0.0
    } else if n % 2 == 0 {
        let mid = n / 2;
        (sorted[mid - 1] as f64 + sorted[mid] as f64) / 2.0
    } else {
        let mid = (n - 1) / 2;
        sorted[mid] as f64
    }
}

fn calculate_four_keys(
    metrics_items: Vec<DeploymentMetricItem>,
    project_config: ProjectConfig,
    context: RetrieveFourKeysExecutionContext,
) -> Result<FourKeysMetrics, RetrieveFourKeysEventError> {
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
        total_deployments as f32 / (days as f32 * (project_config.working_days_per_week / 7.0));

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
        developers: project_config.developer_count,
        working_days_per_week: project_config.working_days_per_week,
        deploys: total_deployments,
        deployment_frequency_per_day,
        deploys_per_a_day_per_a_developer: deployment_frequency_per_day
            / project_config.developer_count as f32,
        lead_time_for_changes: lead_time,
        environment: "production".to_string(), // TODO: get
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

    let deployment_frequency = FourKeysMetrics {
        metrics,
        deployments: deployment_frequencies_by_day,
    };

    Ok(deployment_frequency)
}

// ---------------------------
// overall workflow
// ---------------------------
pub async fn perform<
    FFetchDeploymentsWithGitHubDeployments: FetchDeployments,
    FFetchDeploymentsWithHerokuRelease: FetchDeployments,
    FGetFirstCommitFromCompare: GetFirstCommitFromCompare,
>(
    fetch_deployments_with_github_deployments: FFetchDeploymentsWithGitHubDeployments,
    fetch_deployments_with_heroku_release: FFetchDeploymentsWithHerokuRelease,
    get_first_commit_from_compare: FGetFirstCommitFromCompare,
    project_config: ProjectConfig,
    context: RetrieveFourKeysExecutionContext,
) -> Result<RetrieveFourKeysEvent, RetrieveFourKeysEventError> {
    let deployments = fetch_deployments(
        &fetch_deployments_with_github_deployments,
        &fetch_deployments_with_heroku_release,
        project_config.clone(),
        context.since,
        &context.environment,
    )
    .await?;
    let convert_items = deployments.into_iter().map(|deployment| {
        to_metric_item(
            &get_first_commit_from_compare,
            deployment,
            project_config.clone(),
        )
    });
    let metrics_items = try_join_all(convert_items).await?;
    // .collect::<Result<NonEmptyVec<DeploymentMetricItem>, RetrieveFourKeysEventError>>()?;

    calculate_four_keys(metrics_items, project_config, context)
}
