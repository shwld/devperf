use serde::{Deserialize, Serialize};
use chrono;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentMetrics {
    pub deployments: Vec<DeploymentMetricSummary>,
    pub metrics: DeploymentMetric,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentMetric {
    pub since: chrono::DateTime<chrono::Utc>,
    pub until: chrono::DateTime<chrono::Utc>,
    pub developers: u64,
    pub working_days_per_week: f32,
    pub deploys: u64,
    pub deploys_a_day_a_developer: f32,
    pub deployment_frequency_per_day: f32,
    pub lead_time_for_changes: DeploymentMetricLeadTimeForChanges,
    pub environment: String,
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
    pub date: chrono::NaiveDate,
    pub deploys: u64,
    pub items: Vec<DeploymentMetricItem>,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentItem {
    pub id: String,
    pub commit_sha: String,
    pub commit_message: String,
    pub commit_resource_path: String,
    pub committed_at: chrono::DateTime<chrono::Utc>,
    pub creator_login: String,
    pub deployed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentCommitItem {
    pub sha: String,
    pub message: String,
    pub resource_path: String,
    pub committed_at: chrono::DateTime<chrono::Utc>,
    pub creator_login: String,
}
