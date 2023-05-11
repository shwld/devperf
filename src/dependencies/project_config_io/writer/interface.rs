use async_trait::async_trait;
use thiserror::Error;

use crate::project_creating::dto::ProjectConfigDto;

pub type WriteConfigData = ProjectConfigDto;

#[derive(Error, Debug)]
pub enum ProjectConfigIOWriterError {
    #[error("Cannot write the config file")]
    CannotWritten(#[source] anyhow::Error),
}

#[async_trait]
pub trait ProjectConfigIOWriter {
    async fn write(&self, data: WriteConfigData) -> Result<(), ProjectConfigIOWriterError>;
}
