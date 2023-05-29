use async_trait::async_trait;

use super::retrieve_four_keys::{
    Context, DeploymentFrequency, DeploymentFrequencyLabel,
    DeploymentFrequencyPerformanceSurvey2022, DeploymentPerformanceItem,
    DeploymentPerformanceLeadTimeForChanges, FirstCommitOrRepositoryInfo, FourKeysResult,
    RetrieveFourKeysEvent, RetrieveFourKeysEventError, RetrieveFourKeysExecutionContext,
};
use crate::{
    common_types::date_time_range::DateTimeRange,
    dependencies::deployments_fetcher::interface::DeploymentLog,
};

// ---------------------------
// FetchDeploymentsStep
// ---------------------------
pub(super) struct FetchDeploymentsParams {
    pub timeframe: DateTimeRange,
}
#[async_trait]
pub(super) trait FetchDeploymentsStep {
    async fn fetch_deployments(
        self,
        params: FetchDeploymentsParams,
    ) -> Result<Vec<DeploymentLog>, RetrieveFourKeysEventError>;
}

// ---------------------------
// AttachFirstOperationToDeploymentLogStep
// ---------------------------
#[derive(Debug, Clone)]
pub(super) struct DeploymentLogWithFirstOperation {
    pub(super) deployment: DeploymentLog,
    pub(super) first_operation: Option<FirstCommitOrRepositoryInfo>,
}
#[async_trait]
pub(super) trait AttachFirstOperationToDeploymentLogStep {
    async fn attach_first_operation_to_deployment_item(
        &self,
        deployment_item: DeploymentLog,
    ) -> Result<DeploymentLogWithFirstOperation, RetrieveFourKeysEventError>;
    async fn attach_first_operation_to_deployment_items(
        &self,
        deployment_items: Vec<DeploymentLog>,
    ) -> Result<Vec<DeploymentLogWithFirstOperation>, RetrieveFourKeysEventError>;
}

// ---------------------------
// CalculationEachDeploymentsStep
// ---------------------------
pub(super) type CalculateLeadTimeForChangesSeconds =
    fn(DeploymentLogWithFirstOperation) -> Option<i64>;
pub(super) type ToMetricItem = fn(DeploymentLogWithFirstOperation) -> DeploymentPerformanceItem;

// ---------------------------
// AggregationStep
// ---------------------------
pub(super) type ExtractItemsInPeriod = fn(
    items: Vec<DeploymentPerformanceItem>,
    timeframe: DateTimeRange,
) -> Vec<DeploymentPerformanceItem>;

// TODO: Make the mold more explicit.
pub(super) type CalculateDeploymentFrequency =
    fn(Vec<DeploymentPerformanceItem>, Context) -> DeploymentFrequency;

pub(super) type GetDeploymentPerformance2022 = fn(
    DeploymentFrequency,
    DeploymentFrequencyLabel,
    Context,
) -> DeploymentFrequencyPerformanceSurvey2022;

pub(super) type GetDeploymentPerformanceLabel =
    fn(DeploymentFrequency, Context) -> DeploymentFrequencyLabel;

pub(super) type CalculateLeadTime =
    fn(Vec<DeploymentPerformanceItem>) -> DeploymentPerformanceLeadTimeForChanges;

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
