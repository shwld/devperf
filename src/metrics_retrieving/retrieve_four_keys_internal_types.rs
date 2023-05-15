use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};

use super::retrieve_four_keys::{
    DeploymentMetricItem, FirstCommitOrRepositoryInfo, RetrieveFourKeysEventError,
};
use crate::dependencies::deployments_fetcher::interface::DeploymentItem;

// ---------------------------
// FetchDeploymentsStep
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
// AttachFirstOperationToDeploymentItemStep
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

// ---------------------------
// CalculationEachDeploymentsStep
// ---------------------------
pub(super) type CalculateLeadTimeForChangesSeconds =
    fn(DeploymentItemWithFirstOperation) -> Option<i64>;
pub(super) type ToMetricItem = fn(DeploymentItemWithFirstOperation) -> DeploymentMetricItem;

// ---------------------------
// AggregationStep
// ---------------------------
#[derive(Debug, Clone)]
pub(super) struct DailyItems {
    pub(super) date: NaiveDate,
    pub(super) items: Vec<DeploymentMetricItem>,
}
pub(super) type GroupByDate = fn(Vec<DeploymentMetricItem>) -> Vec<DailyItems>;
