use crate::{dependencies::{read_project_config::interface::{ReadProjectConfig, ProjectConfig, DeploymentSource}, fetch_deployments::interface::{FetchDeployments, FetchDeploymentsParams}}, common_types::DeploymentItem};

use super::{retrieve_four_keys__schema::{RetrieveFourKeysExecutionContext, RetrieveFourKeysEvent, RetrieveFourKeysEventError}};

// ---------------------------
// Fetch deployments step
// ---------------------------

async fn fetch_deployments<TFetchDeploymentsFromGitHubDeployments: FetchDeployments>(project_config: ProjectConfig, fetch_deployments_from_github_deployments: TFetchDeploymentsFromGitHubDeployments, context: RetrieveFourKeysExecutionContext) -> Result<Vec<DeploymentItem>, RetrieveFourKeysEventError> {
    let deployments = match project_config.deployment_source {
        DeploymentSource::GitHubDeployment => {
            fetch_deployments_from_github_deployments.perform(FetchDeploymentsParams {
                owner: project_config.github_owner,
                repo: project_config.github_repo,
                environment: "production".to_string(),
                since: Some(context.since),
            }).await.map_err(RetrieveFourKeysEventError::FetchDeploymentsError)?
        },
        _ => unimplemented!(),
    };

    Ok(deployments)
}

// ---------------------------
// Calculation step
// ---------------------------


// ---------------------------
// overall workflow
// ---------------------------
pub async fn perform<TReadProjectConfig: ReadProjectConfig, TFetchDeploymentsFromGitHubDeployments: FetchDeployments>(read_config: TReadProjectConfig, fetch_deployments_from_github_deployments: TFetchDeploymentsFromGitHubDeployments, context: RetrieveFourKeysExecutionContext) -> Result<RetrieveFourKeysEvent, RetrieveFourKeysEventError> {
    let project_config = read_config.perform(context.clone().project_name).await.map_err(RetrieveFourKeysEventError::ReadProjectConfigError)?;
    let deployments = fetch_deployments(project_config.clone(), fetch_deployments_from_github_deployments, context).await?;

    unimplemented!()
}
