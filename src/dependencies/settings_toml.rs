use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common_types::ProjectConfig;

pub type ProjectName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub github_personal_token: String,
    pub projects: HashMap<ProjectName, ProjectConfig>,
}
