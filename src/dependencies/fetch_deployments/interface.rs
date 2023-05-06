use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::{common_types::DeploymentItem};

pub struct FetchDeploymentsParams {
    pub owner: String,
    pub repo: String,
    pub environment: String,
    pub since: Option<DateTime<Utc>>,
}

pub struct FetchDeploymentsError(pub String);

#[async_trait]
pub trait FetchDeployments {
    async fn perform(&self, params: FetchDeploymentsParams) -> Result<Vec<DeploymentItem>, FetchDeploymentsError>;
}
