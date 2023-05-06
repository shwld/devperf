use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

pub struct NonZeroU32(u32);
impl NonZeroU32 {
    pub fn new(number: u32) -> Result<Self, String> {
        if number > 0 {
            Ok(NonZeroU32(number))
        } else {
            Err("Integer must be greater than zero".to_string())
        }
    }
}

pub struct NonZeroF32(f32);
impl NonZeroF32 {
    pub fn new(number: f32) -> Result<Self, String> {
        if number > 0.0 {
            Ok(NonZeroF32(number))
        } else {
            Err("Integer must be greater than zero".to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentItem {
    pub id: String,
    pub head_commit_sha: String,
    pub head_commit_message: String,
    pub head_commit_resource_path: String,
    pub head_committed_at: DateTime<Utc>,
    pub creator_login: String,
    pub deployed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub github_personal_token: Option<String>,
    pub github_owner: String,
    pub github_repo: String,
    pub developer_count: u32,
    pub working_days_per_week: f32,
    pub deployment_source: String,
}
