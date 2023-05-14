use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::retrieve_four_keys::{FirstCommitOrRepositoryInfo, RetrieveFourKeysEventError};
use crate::dependencies::deployments_fetcher::interface::DeploymentItem;

// ---------------------------
// Fetch deployments step
// ---------------------------
pub(super) struct FetchDeploymentsParams {
    pub(super) since: DateTime<Utc>,
    pub(super) until: DateTime<Utc>,
}
#[async_trait]
pub(super) trait FetchDeploymentsStep {
    async fn fetch_deployments(
        self,
        params: FetchDeploymentsParams,
    ) -> Result<Vec<DeploymentItem>, RetrieveFourKeysEventError>;
}

// ---------------------------
// Fetch deployments step
// ---------------------------
#[derive(Debug, Clone)]
pub(super) struct DeploymentItemWithFirstOperation {
    pub(super) deployment: DeploymentItem,
    pub(super) first_operation: Option<FirstCommitOrRepositoryInfo>,
}
#[async_trait]
pub(super) trait AttachFirstOperationToDeploymentItemStep {
    async fn attach_first_operation_to_deployment_item(
        &self,
        deployment_item: DeploymentItem,
    ) -> Result<DeploymentItemWithFirstOperation, RetrieveFourKeysEventError>;
    async fn attach_first_operation_to_deployment_items(
        &self,
        deployment_items: Vec<DeploymentItem>,
    ) -> Result<Vec<DeploymentItemWithFirstOperation>, RetrieveFourKeysEventError>;
}
