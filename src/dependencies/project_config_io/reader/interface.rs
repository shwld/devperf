use async_trait::async_trait;
use thiserror::Error;

use crate::project_creating::dto::ProjectConfigDto;

#[derive(Debug, Error)]
pub enum ProjectConfigIOReaderError {
    #[error("Cannot read the config file")]
    ConfigFileReadError(#[source] anyhow::Error),
    #[error("Cannot find the project")]
    ProjectNotFound(String),
    #[error("Cannot read heroku app name")]
    CannotReadHerokuAppName(String),
    #[error("Cannot read heroku api token")]
    CannotReadHerokuAuthToken(String),
}

#[async_trait]
pub trait ProjectConfigIOReader {
    async fn read(
        &self,
        project_name: String,
    ) -> Result<ProjectConfigDto, ProjectConfigIOReaderError>;
}