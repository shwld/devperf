use std::collections::HashMap;
use cranenum::Cranenum;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub github_owner: String,
    pub github_repo: String,
    pub developer_count: u32,
    pub working_days_per_week: f32,
    pub deployment_source: String,
}

type ProjectName = String;

#[derive(Debug, Clone, Serialize)]
pub struct Config {
    pub github_personal_token: String,
    pub projects: HashMap<ProjectName, ProjectConfig>,
}

pub struct WriteNewConfigData {
    pub project_name: String,
    pub github_personal_token: String,
    pub project_config: ProjectConfig,
}

#[derive(Error, Debug)]
pub enum WriteNewConfigError {
    #[error("Cannot write the new config")]
    ConfigFileWriteError
}

pub trait WriteNewConfig {
    fn perform(&self, params: WriteNewConfigData) -> Result<(), WriteNewConfigError>;
}
