use std::collections::HashMap;

use async_trait::async_trait;
use chrono::NaiveDate;

use super::retrieve_four_keys::{
    Context, DeploymentFrequency, DeploymentFrequencyLabel,
    DeploymentFrequencyPerformanceSurvey2022, DeploymentPerformanceItem,
    DeploymentPerformanceLeadTimeForChanges, FirstCommitOrRepositoryInfo, FourKeysResult,
    RetrieveFourKeysEvent, RetrieveFourKeysEventError, RetrieveFourKeysExecutionContext,
};
use crate::{
    common_types::date_time_range::DateTimeRange,
    dependencies::deployments_fetcher::interface::DeploymentItem,
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
    timeframe: DateTimeRange,
) -> Vec<DeploymentPerformanceItem>;

#[derive(Debug, Clone)]
pub(super) struct DailyItems(pub(super) HashMap<NaiveDate, Vec<DeploymentPerformanceItem>>);
#[derive(Debug, Clone)]
pub(super) struct WeeklyItems(pub(super) HashMap<NaiveDate, Vec<DeploymentPerformanceItem>>);
#[derive(Debug, Clone)]
pub(super) struct MonthlyItems(pub(super) HashMap<u32, Vec<DeploymentPerformanceItem>>);

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
