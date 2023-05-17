use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::dependencies::{
    deployments_fetcher::interface::{DeploymentInfo, DeploymentsFetcherError},
    first_commit_getter::interface::FirstCommitGetterError,
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
    pub since: DateTime<Utc>,
    pub until: DateTime<Utc>,
}

// ------------------------------------
// outputs from the workflow (success case)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentCommitItem {
    pub sha: String,
    pub message: String,
    pub resource_path: String,
    pub committed_at: chrono::DateTime<chrono::Utc>,
    pub creator_login: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FirstCommitOrRepositoryInfo {
    FirstCommit(DeploymentCommitItem),
    RepositoryInfo(RepositoryInfo),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentPerformanceItem {
    pub info: DeploymentInfo,
    pub head_commit: DeploymentCommitItem,
    pub first_commit: FirstCommitOrRepositoryInfo,
    pub deployed_at: chrono::DateTime<chrono::Utc>,
    pub lead_time_for_changes_seconds: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentPerformanceSummary {
    pub date: NaiveDate,
    pub deploys: u32,
    pub items: Vec<DeploymentPerformanceItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentPerformanceLeadTimeForChanges {
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub total_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Context {
    pub since: DateTime<chrono::Utc>,
    pub until: chrono::DateTime<chrono::Utc>,
    pub developers: u32,
    pub working_days_per_week: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentPerformance {
    pub deploys: u32,
    pub deploys_per_a_day_per_a_developer: f32,
    pub deployment_frequency_per_day: f32,
    pub lead_time_for_changes: DeploymentPerformanceLeadTimeForChanges,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FourKeysResult {
    pub deployments: Vec<DeploymentPerformanceSummary>,
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
    FetchDeploymentsError(#[from] DeploymentsFetcherError),
    #[error("GetFirstCommitFromCompareError")]
    GetFirstCommitFromCompareError(#[from] FirstCommitGetterError),
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
