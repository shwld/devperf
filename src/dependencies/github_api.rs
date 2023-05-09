use octocrab::{Octocrab};
use thiserror::Error;

use super::read_project_config::interface::{ProjectConfig, ResourceConfig};

#[derive(Clone)]
pub struct GitHubAPI {
    pub project_config: ProjectConfig,
}
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

impl GitHubAPI {
    pub fn get_client(self) -> Result<Octocrab, GitHubClientError> {
        let token = match &self.project_config.resource {
            ResourceConfig::GitHubDeployment(resource_config) => resource_config.github_personal_token.clone(),
            ResourceConfig::HerokuRelease(resource_config) => resource_config.github_personal_token.clone(),
            _ => unimplemented!(),
        };
        let octocrab = Octocrab::builder()
            .personal_token(token)
            .build()?;

        Ok(octocrab)
    }
}
