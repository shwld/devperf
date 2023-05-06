use async_trait::async_trait;
use thiserror::Error;

use crate::common_types::ConfigData;

#[derive(Error, Debug)]
pub enum WriteNewConfigError {
    #[error("Cannot write the new config")]
    ConfigFileWriteError
}

#[async_trait]
pub trait WriteNewConfig {
    async fn perform(&self, params: ConfigData) -> Result<(), WriteNewConfigError>;
}
