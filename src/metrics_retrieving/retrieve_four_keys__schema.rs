use chrono::{DateTime, Utc, NaiveDate};
use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::dependencies::{read_project_config::interface::{ProjectConfig}, fetch_deployments::interface::FetchDeploymentsError, get_first_commit_from_compare::interface::GetFirstCommitFromCompareError};

// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the workflow
// ==================================

// ------------------------------------
// inputs to the workflow

#[derive(Clone)]
pub struct RetrieveFourKeysExecutionContext {
    pub project_name: String,
    pub since: DateTime<Utc>,
    pub until: DateTime<Utc>,
    pub environment: String,
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
#[non_exhaustive]
pub struct DeploymentMetricItem {
    pub id: String,
    pub head_commit: DeploymentCommitItem,
    pub first_commit: DeploymentCommitItem,
    pub deployed_at: chrono::DateTime<chrono::Utc>,
    pub lead_time_for_changes_seconds: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentMetricSummary {
    pub date: NaiveDate,
    pub deploys: u32,
    pub items: Vec<DeploymentMetricItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentMetricLeadTimeForChanges {
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub total_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentMetric {
    pub since: DateTime<chrono::Utc>,
    pub until: chrono::DateTime<chrono::Utc>,
    pub developers: u32,
    pub working_days_per_week: f32,
    pub deploys: u32,
    pub deploys_per_a_day_per_a_developer: f32,
    pub deployment_frequency_per_day: f32,
    pub lead_time_for_changes: DeploymentMetricLeadTimeForChanges,
    pub environment: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FourKeysMetrics {
    pub deployments: Vec<DeploymentMetricSummary>,
    pub metrics: DeploymentMetric,
}

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub type RetrieveFourKeysEvent = FourKeysMetrics;

// Error types
#[derive(Error, Debug)]
pub enum RetrieveFourKeysEventError {
    #[error("Cannot fetch")]
    FetchDeploymentsError(#[source] FetchDeploymentsError),
    #[error("GetFirstCommitFromCompareError")]
    GetFirstCommitFromCompareError(#[from] GetFirstCommitFromCompareError),
    #[error("Cannot get date")]
    CannotGetDate(String),
}

// ------------------------------------
// the workflow itself
pub type RetrieveFourKeys = fn (ProjectConfig, RetrieveFourKeysExecutionContext) -> Result<RetrieveFourKeysEvent, RetrieveFourKeysEventError>;
