use chrono::{DateTime, Utc};

use crate::{dependencies::{read_project_config::interface::{ReadProjectConfig, ProjectConfig, DeploymentSource}, fetch_deployments::interface::{FetchDeployments, FetchDeploymentsParams, DeploymentItem}, get_first_commit_from_compare::interface::{GetFirstCommitFromCompare, FirstCommitFromCompareParams}}, common_types::{NonEmptyVec}};

use super::{retrieve_four_keys__schema::{RetrieveFourKeysExecutionContext, RetrieveFourKeysEvent, RetrieveFourKeysEventError, DeploymentMetricItem, DeploymentCommitItem}};

// ---------------------------
// Fetch deployments step
// ---------------------------

async fn fetch_deployments<F: FetchDeployments>(fetch_deployments_from_github_deployments: F, project_config: ProjectConfig, since: DateTime<Utc>) -> Result<Vec<DeploymentItem>, RetrieveFourKeysEventError> {
    let deployments = match project_config.deployment_source {
        DeploymentSource::GitHubDeployment => {
            fetch_deployments_from_github_deployments.perform(FetchDeploymentsParams {
                owner: project_config.github_owner,
                repo: project_config.github_repo,
                environment: "production".to_string(),
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

pub async fn to_metric_item<F: GetFirstCommitFromCompare>(get_first_commit_from_compare: F, deployment: DeploymentItem, project_config: ProjectConfig) -> Result<DeploymentMetricItem, RetrieveFourKeysEventError> {
    let first_commit = get_first_commit_from_compare.perform(FirstCommitFromCompareParams {
        owner: project_config.github_owner,
        repo: project_config.github_repo,
        base: deployment.base_commit.sha,
        head: deployment.head_commit.sha.clone(),
    }).await?;
    let head_commit = deployment.head_commit.clone();
    let lead_time_for_changes_seconds = (deployment.deployed_at - first_commit.committed_at).num_seconds();
    let deployment_metric = DeploymentMetricItem {
        id: deployment.id,
        head_commit: DeploymentCommitItem {
            sha: head_commit.sha,
            message: head_commit.message,
            resource_path: head_commit.resource_path,
            committed_at: head_commit.committed_at,
            creator_login: head_commit.creator_login,
        },
        first_commit: DeploymentCommitItem {
            sha: first_commit.sha,
            message: first_commit.message,
            resource_path: first_commit.resource_path,
            committed_at: first_commit.committed_at,
            creator_login: first_commit.creator_login,
        },
        deployed_at: deployment.deployed_at,
        lead_time_for_changes_seconds: lead_time_for_changes_seconds,
    };

    Ok(deployment_metric)
}


// ---------------------------
// Calculation step
// ---------------------------


// ---------------------------
// overall workflow
// ---------------------------
pub async fn perform<TReadProjectConfig: ReadProjectConfig, TFetchDeploymentsFromGitHubDeployments: FetchDeployments>(read_config: TReadProjectConfig, fetch_deployments_from_github_deployments: TFetchDeploymentsFromGitHubDeployments, context: RetrieveFourKeysExecutionContext) -> Result<RetrieveFourKeysEvent, RetrieveFourKeysEventError> {
    let project_config = read_config.perform(context.clone().project_name).await.map_err(RetrieveFourKeysEventError::ReadProjectConfigError)?;
    let deployments = fetch_deployments(fetch_deployments_from_github_deployments, project_config.clone(), context.since).await?;

    unimplemented!()
}
