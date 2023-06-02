use async_trait::async_trait;

use super::retrieve_four_keys::{
    Context, Deployment, DeploymentFrequency, DeploymentFrequencyLabel,
    DeploymentFrequencyPerformanceSurvey2022, DeploymentLeadTimeForChanges,
    FirstCommitOrRepositoryInfo, FourKeysResult, RetrieveFourKeysEvent, RetrieveFourKeysEventError,
    RetrieveFourKeysExecutionContext,
};
use crate::{
    common_types::commit::Commit, dependencies::deployments_fetcher::interface::DeploymentLog,
};

// ---------------------------
// PickFirstCommit
// ---------------------------
pub(super) type PickFirstCommit = fn(commits: &Vec<Commit>) -> Option<Commit>;

// ---------------------------
// CalculateEachLogLeadTimes
// ---------------------------
#[derive(Debug, Clone)]
pub(super) struct DeploymentLogWithFirstOperation {
    pub(super) deployment_log: DeploymentLog,
    pub(super) first_operation: Option<FirstCommitOrRepositoryInfo>,
}
pub(super) type CalculateLeadTime = fn(DeploymentLogWithFirstOperation) -> Deployment;

// ---------------------------
// Aggregation
// ---------------------------
pub(super) type CalculateDeploymentFrequency = fn(Vec<Deployment>, &Context) -> DeploymentFrequency;

pub(super) type CalculateDeploymentFrequencyPerDay = fn(&Vec<Deployment>, &Context) -> f32;

pub(super) type GetDeploymentPerformance2022 = fn(
    &DeploymentFrequency,
    &DeploymentFrequencyLabel,
    &Context,
) -> DeploymentFrequencyPerformanceSurvey2022;

pub(super) type GetDeploymentPerformanceLabel =
    fn(&DeploymentFrequency, &Context) -> DeploymentFrequencyLabel;

pub(super) type CalculateLeadTimeMedian = fn(&Vec<Deployment>) -> DeploymentLeadTimeForChanges;

// ---------------------------
// RetrieveFourKeys
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
