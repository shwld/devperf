use octocrab::Octocrab;
use thiserror::Error;

use crate::common_types::{
    validate_github_owner_repo::ValidatedGitHubOwnerRepo,
    validate_github_personal_token::ValidatedGitHubPersonalToken,
};

#[derive(Error, Debug)]
pub enum GetCreatedAtError {
    #[error("GitHub API Client error")]
    ClientError(#[from] octocrab::Error),
    #[error("Repository cannot fetch")]
    RepositoryCannotFetch(#[source] anyhow::Error),
}

pub(super) async fn get_created_at(
    github_personal_token: ValidatedGitHubPersonalToken,
    github_owner_repo: ValidatedGitHubOwnerRepo,
) -> Result<chrono::DateTime<chrono::Utc>, GetCreatedAtError> {
    let octocrab = Octocrab::builder()
        .personal_token(github_personal_token.to_string())
        .build()?;
    let result = octocrab
        .repos(github_owner_repo.get_owner(), github_owner_repo.get_repo())
        .get()
        .await
        .map(|r| r.created_at)
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(GetCreatedAtError::RepositoryCannotFetch)?;
    let created_at = result.ok_or(GetCreatedAtError::RepositoryCannotFetch(anyhow::anyhow!(
        "Cannot fetch repository: {}",
        github_owner_repo.to_string()
    )))?;

    Ok(created_at)
}
