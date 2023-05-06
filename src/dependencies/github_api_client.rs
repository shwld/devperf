use octocrab::{Octocrab};
use thiserror::Error;

pub type GitHubAPIClient = Octocrab;
#[derive(Error, Debug)]
pub enum GitHubClientError {
    #[error("Octocrab error")]
    OctocrabError(#[from] octocrab::Error),
    #[error("Cannot get the GitHub personal token")]
    GetGitHubPersonalTokenError(#[from] GetGitHubPersonalTokenError),
}

#[derive(Error, Debug)]
pub enum GetGitHubPersonalTokenError {
    #[error("Cannot get the GitHub personal token")]
    GetGitHubPersonalTokenError(#[source] anyhow::Error),
}
pub type GetGitHubPersonalToken = fn () -> Result<String, GetGitHubPersonalTokenError>;

pub fn create_github_client(get_personal_token: GetGitHubPersonalToken) -> Result<GitHubAPIClient, GitHubClientError> {
    let token = get_personal_token()?;
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()?;

    Ok(octocrab)
}
