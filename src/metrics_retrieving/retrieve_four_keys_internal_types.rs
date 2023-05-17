use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};

use super::retrieve_four_keys::{
    DeploymentPerformanceItem, DeploymentPerformanceLeadTimeForChanges,
    FirstCommitOrRepositoryInfo, FourKeysResult, RetrieveFourKeysEvent, RetrieveFourKeysEventError,
    RetrieveFourKeysExecutionContext,
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
pub(super) type ToMetricItem = fn(DeploymentItemWithFirstOperation) -> DeploymentPerformanceItem;

// ---------------------------
// AggregationStep
// ---------------------------
pub(super) type ExtractItemsInPeriod = fn(
    items: Vec<DeploymentPerformanceItem>,
    since: DateTime<Utc>,
    until: DateTime<Utc>,
) -> Vec<DeploymentPerformanceItem>;

#[derive(Debug, Clone)]
pub(super) struct DailyItems {
    pub(super) date: NaiveDate,
    pub(super) items: Vec<DeploymentPerformanceItem>,
}

pub(super) type GroupByDate = fn(Vec<DeploymentPerformanceItem>) -> Vec<DailyItems>;

pub(super) type CalculateTotalDeployments = fn(Vec<DailyItems>) -> u32;

// TODO: Make the mold more explicit.
pub(super) type CalculateDeploymentFrequencyPerDay = fn(
    total_deployments: u32,
    since: DateTime<Utc>,
    until: DateTime<Utc>,
    working_days_per_week: f32,
) -> f32;

pub(super) type CalculateLeadTime = fn(Vec<DailyItems>) -> DeploymentPerformanceLeadTimeForChanges;

// ---------------------------
// RetrieveFourKeysStep
// ---------------------------
#[async_trait]
pub(super) trait RetrieveFourKeysStep {
    async fn retrieve_four_keys(
        self,
        context: RetrieveFourKeysExecutionContext,
    ) -> Result<FourKeysResult, RetrieveFourKeysEventError>;
}

// ---------------------------
// Create events
// ---------------------------
pub(super) type CreateEvents = fn(project: FourKeysResult) -> Vec<RetrieveFourKeysEvent>;
