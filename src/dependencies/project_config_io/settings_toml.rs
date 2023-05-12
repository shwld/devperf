use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type ProjectName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub github_personal_token: Option<String>,
    pub github_owner: String,
    pub github_repo: String,
    pub github_deployment_environment: Option<String>,
    pub heroku_app_name: Option<String>,
    pub heroku_auth_token: Option<String>,
    pub developer_count: u32,
    pub working_days_per_week: f32,
    pub deployment_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub github_personal_token: String,
    pub projects: HashMap<ProjectName, ProjectConfig>,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            github_personal_token: "".to_string(),
            projects: HashMap::new(),
        }
    }
}
