use cranenum::Cranenum;
use octocrab::{Octocrab};

pub type GitHubAPIClient = Octocrab;
#[derive(Cranenum)]
pub enum GitHubClientError {
    OctocrabError(octocrab::Error),
    GetGitHubPersonalTokenError(GetGitHubPersonalTokenError),
}

pub struct GetGitHubPersonalTokenError(pub String);
pub type GetGitHubPersonalToken = fn () -> Result<String, GetGitHubPersonalTokenError>;

pub fn create_github_client(get_personal_token: GetGitHubPersonalToken) -> Result<GitHubAPIClient, GitHubClientError> {
    let token = get_personal_token()?;
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()?;

    Ok(octocrab)
}
