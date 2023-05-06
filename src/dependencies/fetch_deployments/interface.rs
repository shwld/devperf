use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::{common_types::DeploymentItem};

pub struct FetchDeploymentsParams {
    pub owner: String,
    pub repo: String,
    pub environment: String,
    pub since: Option<DateTime<Utc>>,
}

#[derive(Debug, Error)]
pub enum FetchDeploymentsError {
    #[error("Cannot read the config file")]
    FetchDeploymentsError(#[source] anyhow::Error),
}

#[async_trait]
pub trait FetchDeployments {
    async fn perform(&self, params: FetchDeploymentsParams) -> Result<Vec<DeploymentItem>, FetchDeploymentsError>;
}
