use chrono::{DateTime, Utc, NaiveTime, LocalResult, NaiveDate};
use futures::future::{try_join_all};

use crate::{dependencies::{read_project_config::interface::{ReadProjectConfig, ProjectConfig, DeploymentSource, ResourceConfig}, fetch_deployments::interface::{FetchDeployments, FetchDeploymentsParams, DeploymentItem, CommitOrRepositoryInfo}, get_first_commit_from_compare::interface::{GetFirstCommitFromCompare, FirstCommitFromCompareParams}}, common_types::{NonEmptyVec}, metrics_retrieving::retrieve_four_keys__schema::FirstCommitOrRepositoryInfo};

use super::{retrieve_four_keys__schema::{RetrieveFourKeysExecutionContext, RetrieveFourKeysEvent, RetrieveFourKeysEventError, DeploymentMetricItem, DeploymentCommitItem, DeploymentMetric, FourKeysMetrics, DeploymentMetricLeadTimeForChanges, DeploymentMetricSummary, RepositoryInfo}};

// ---------------------------
// Fetch deployments step
// ---------------------------

async fn fetch_deployments<F: FetchDeployments>(fetch_deployments_from_github_deployments: &F, project_config: ProjectConfig, since: DateTime<Utc>, environment: &str) -> Result<Vec<DeploymentItem>, RetrieveFourKeysEventError> {
    let deployments = match project_config.resource {
        ResourceConfig::GitHubDeployment(resource_config) => {
            fetch_deployments_from_github_deployments.perform(FetchDeploymentsParams {
                owner: resource_config.github_owner,
                repo: resource_config.github_repo,
                environment: environment.to_string(),
                since: Some(since),
            }).await.map_err(RetrieveFourKeysEventError::FetchDeploymentsError)
        },
        _ => unimplemented!(),
    }?;

    Ok(deployments)
}

// ---------------------------
// Convert to MetricItem step
// ---------------------------

pub async fn to_metric_item<F: GetFirstCommitFromCompare>(get_first_commit_from_compare: &F, deployment: DeploymentItem, project_config: ProjectConfig) -> Result<DeploymentMetricItem, RetrieveFourKeysEventError> {
    let first_commit = match deployment.base {
        CommitOrRepositoryInfo::Commit(first_commit) => {
            let commit = get_first_commit_from_compare.perform(match project_config.resource {
                ResourceConfig::GitHubDeployment(resource_config) => {
                    FirstCommitFromCompareParams {
                        owner: resource_config.github_owner,
                        repo: resource_config.github_repo,
                        base: first_commit.sha,
                        head: deployment.head_commit.sha.clone(),
                    }},
                    _ => unimplemented!(),
            }).await?;
            FirstCommitOrRepositoryInfo::FirstCommit(DeploymentCommitItem {
                sha: commit.sha,
                message: commit.message,
                resource_path: commit.resource_path,
                committed_at: commit.committed_at,
                creator_login: commit.creator_login,
            })
        },
        CommitOrRepositoryInfo::RepositoryInfo(info) => {
            FirstCommitOrRepositoryInfo::RepositoryInfo(RepositoryInfo { created_at: info.created_at })
        },
    };
    let first_committed_at = match first_commit.clone() {
        FirstCommitOrRepositoryInfo::FirstCommit(commit) => commit.committed_at,
        FirstCommitOrRepositoryInfo::RepositoryInfo(info) => info.created_at,
    };

    let head_commit = deployment.head_commit.clone();
    let lead_time_for_changes_seconds = (deployment.deployed_at - first_committed_at).num_seconds();
    let deployment_metric = DeploymentMetricItem {
        id: deployment.id,
        head_commit: DeploymentCommitItem {
            sha: head_commit.sha,
            message: head_commit.message,
            resource_path: head_commit.resource_path,
            committed_at: head_commit.committed_at,
            creator_login: head_commit.creator_login,
        },
        first_commit: first_commit,
        deployed_at: deployment.deployed_at,
        lead_time_for_changes_seconds: lead_time_for_changes_seconds,
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
    if inner_items.len() > 0 {
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

fn calculate_four_keys(metrics_items: Vec<DeploymentMetricItem>, project_config: ProjectConfig, context: RetrieveFourKeysExecutionContext) -> Result<FourKeysMetrics, RetrieveFourKeysEventError> {
    let ranged_items = metrics_items.into_iter().filter(|it| it.deployed_at >= context.since && it.deployed_at <= context.until).collect::<Vec<DeploymentMetricItem>>();
    let items_by_day = split_by_day(ranged_items);

    let total_deployments = items_by_day.iter().fold(0, |total, i| total + i.len() as u32);
    let duration_since = context.until.signed_duration_since(context.since);
    let days = duration_since.num_days();
    let deployment_frequency_per_day = total_deployments as f32 / (days as f32 * (project_config.working_days_per_week / 7.0));

    let durations = items_by_day.iter().flat_map(|items| {
        items
            .iter()
    }).map(|item| item.lead_time_for_changes_seconds).collect::<Vec<i64>>();
    let median_duration = median(durations);
    let hours = (median_duration / 3600.0) as i64;
    let minutes = ((median_duration.round() as i64 % 3600) / 60) as i64;
    let seconds = ((median_duration.round() as i64) - (hours * 3600) - (minutes * 60)) as i64;
    let lead_time = DeploymentMetricLeadTimeForChanges {
        hours: hours,
        minutes: minutes,
        seconds: seconds,
        total_seconds: median_duration,
    };

    let metrics = DeploymentMetric {
        since: context.since,
        until: context.until,
        developers: project_config.developer_count,
        working_days_per_week: project_config.working_days_per_week,
        deploys: total_deployments,
        deployment_frequency_per_day: deployment_frequency_per_day,
        deploys_per_a_day_per_a_developer: deployment_frequency_per_day / project_config.developer_count as f32,
        lead_time_for_changes: lead_time,
        environment: "production".to_string(), // TODO: get
    };

    let deployment_frequencies_by_day = items_by_day.into_iter().map(|items| {
        // TODO: 型定義でTotalityを確保したい
        let date = items[0].deployed_at.date_naive();
        let deployments = items.len() as u32;
        DeploymentMetricSummary {
            date: date,
            deploys: deployments,
            items: items,
        }
    }).collect::<Vec<DeploymentMetricSummary>>();

    let deployment_frequency = FourKeysMetrics {
        metrics: metrics,
        deployments: deployment_frequencies_by_day,
    };

    Ok(deployment_frequency)
}

// ---------------------------
// overall workflow
// ---------------------------
pub async fn perform<
    FFetchDeploymentsFromGitHubDeployments: FetchDeployments,
    FGetFirstCommitFromCompare: GetFirstCommitFromCompare,
>(
    fetch_deployments_from_github_deployments: FFetchDeploymentsFromGitHubDeployments,
    get_first_commit_from_compare: FGetFirstCommitFromCompare,
    project_config: ProjectConfig,
    context: RetrieveFourKeysExecutionContext
) -> Result<RetrieveFourKeysEvent, RetrieveFourKeysEventError> {
    let deployments = fetch_deployments(&fetch_deployments_from_github_deployments, project_config.clone(), context.since, &context.environment).await?;
    let convert_items = deployments.into_iter().map(|deployment| {
        to_metric_item(&get_first_commit_from_compare, deployment, project_config.clone())
    });
    let metrics_items = try_join_all(convert_items).await?;
    // .collect::<Result<NonEmptyVec<DeploymentMetricItem>, RetrieveFourKeysEventError>>()?;

    let result = calculate_four_keys(metrics_items, project_config, context);

    result
}
