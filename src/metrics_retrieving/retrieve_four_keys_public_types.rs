use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    common_types::{commit::Commit, date_time_range::DateTimeRange},
    dependencies::{
        deployments_fetcher::interface::{DeploymentInfo, DeploymentsFetcherError},
        two_commits_comparer::interface::{TwoCommitsComparerError, ValidatedCommitShaPairError},
    },
};

// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the workflow
// ==================================

// ------------------------------------
// inputs to the workflow

#[derive(Clone)]
pub struct RetrieveFourKeysExecutionContextProject {
    pub name: String,
    pub developer_count: u32,
    pub working_days_per_week: f32,
}

#[derive(Clone)]
pub struct RetrieveFourKeysExecutionContext {
    pub project: RetrieveFourKeysExecutionContextProject,
    pub timeframe: DateTimeRange,
}

// ------------------------------------
// outputs from the workflow (success case)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FirstCommitOrRepositoryInfo {
    FirstCommit(Commit),
    RepositoryInfo(RepositoryInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Deployment {
    pub info: DeploymentInfo,
    pub head_commit: Commit,
    pub first_commit: FirstCommitOrRepositoryInfo,
    pub deployed_at: chrono::DateTime<chrono::Utc>,
    pub lead_time_for_changes_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DailyDeploymentsSummary {
    pub date: NaiveDate,
    pub deploys: u32,
    pub items: Vec<Deployment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentLeadTimeForChanges {
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub total_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Context {
    pub timeframe: DateTimeRange,
    pub developers: u32,
    pub working_days_per_week: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DeploymentFrequencyPerformanceSurvey2022 {
    Elite,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DeploymentFrequencyLabel {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentFrequency {
    pub total_deployments: u32,
    pub weekly_deployment_count_median: f64,
    pub week_deployed_median: f64,
    pub month_deployed_median: f64,
    pub deployment_frequency_per_day: f32,
    pub deploys_per_a_day_per_a_developer: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentFrequencyPerformance {
    pub performance: DeploymentFrequencyPerformanceSurvey2022,
    pub label: DeploymentFrequencyLabel,
    pub value: DeploymentFrequency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentPerformance {
    pub deployment_frequency: DeploymentFrequencyPerformance,
    pub lead_time_for_changes: DeploymentLeadTimeForChanges,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FourKeysResult {
    pub deployments: Vec<DailyDeploymentsSummary>,
    pub context: Context,
    pub performance: DeploymentPerformance,
}

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub enum RetrieveFourKeysEvent {
    RetrieveFourKeys(FourKeysResult),
}

// Error types
#[derive(Error, Debug)]
pub enum RetrieveFourKeysEventError {
    #[error("Cannot fetch")]
    FetchDeployments(#[from] DeploymentsFetcherError),
    #[error("GetFirstCommitFromCompareError: {0}")]
    TwoCommitsCompare(#[from] TwoCommitsComparerError),
}

// ------------------------------------
// the workflow itself
#[async_trait]
pub trait RetrieveFourKeys {
    async fn retrieve_four_keys(
        self,
        context: RetrieveFourKeysExecutionContext,
    ) -> Result<Vec<RetrieveFourKeysEvent>, RetrieveFourKeysEventError>;
}
