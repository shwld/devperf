use crate::{
    dependencies::github_api::GitHubAPI,
    project_parameter_validating::validate_github_owner_repo::ValidatedGitHubOwnerRepo,
};

use super::interface::DeploymentsFetcherError;

pub(super) async fn get_created_at(
    github_api: GitHubAPI,
    github_owner_repo: ValidatedGitHubOwnerRepo,
) -> Result<chrono::DateTime<chrono::Utc>, DeploymentsFetcherError> {
    let result = github_api
        .get_client()
        .repos(github_owner_repo.get_owner(), github_owner_repo.get_repo())
        .get()
        .await
        .map(|r| r.created_at)
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(DeploymentsFetcherError::GetRepositoryCreatedAtError)?;
    let created_at = result.ok_or(DeploymentsFetcherError::RepositoryNotFound(
        github_owner_repo.to_string(),
    ))?;

    Ok(created_at)
}
