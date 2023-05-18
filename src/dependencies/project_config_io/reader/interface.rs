use async_trait::async_trait;
use thiserror::Error;

use crate::{
    common_types::{
        github_personal_token::ValidatedGitHubPersonalToken,
        heroku_auth_token::ValidatedHerokuAuthToken,
    },
    project_creating::dto::ProjectConfigDto,
};

pub struct GlobalConfig {
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub heroku_auth_token: Option<ValidatedHerokuAuthToken>,
}

#[derive(Debug, Error)]
pub enum ProjectConfigIOReaderError {
    #[error("Cannot read the config file")]
    ConfigFileReadError(#[source] anyhow::Error),
    #[error("Cannot find the project")]
    ProjectNotFound(String),
    #[error("Invalid data source")]
    DataSourceIsInvalid(String),
}

#[async_trait]
pub trait ProjectConfigIOReader {
    async fn read(
        &self,
        project_name: String,
    ) -> Result<ProjectConfigDto, ProjectConfigIOReaderError>;
    async fn read_globals(&self) -> Result<GlobalConfig, ProjectConfigIOReaderError>;
}
