use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub github_personal_token: Option<String>,
    pub github_owner: String,
    pub github_repo: String,
    pub heroku_app_name: Option<String>,
    pub heroku_api_token: Option<String>,
    pub developer_count: u32,
    pub working_days_per_week: f32,
    pub deployment_source: String,
}
