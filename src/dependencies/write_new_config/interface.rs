use async_trait::async_trait;
use thiserror::Error;

use crate::common_types::ProjectConfig;

#[derive(Debug, Clone)]
pub struct WriteConfigData {
    pub project_name: String,
    pub github_personal_token: String,
    pub project_config: ProjectConfig,
}

#[derive(Error, Debug)]
pub enum WriteNewConfigError {
    #[error("Cannot write the new config")]
    ConfigFileWriteError(#[source] anyhow::Error),
}

#[async_trait]
pub trait WriteNewConfig {
    async fn perform(&self, params: WriteConfigData) -> Result<(), WriteNewConfigError>;
}
