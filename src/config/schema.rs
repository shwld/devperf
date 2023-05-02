use serde::{Serialize, Deserialize};
use std::{collections::HashMap, fmt};

#[derive(Debug, Serialize, Deserialize)]
pub enum DeploymentSource {
    GitHubDeployment,
    HerokuRelease,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectAccessToken {
  UseGlobal,
  String(String),
}

// Access tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct UnvalidatedAccessTokens {
    pub github_personal_token: Option<String>,
    pub heroku_token: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatedGitHubDeploymentAccessTokens {
    pub github_personal_token: ProjectAccessToken,
    pub heroku_token: Option<ProjectAccessToken>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatedHerokuReleasesAccessTokens {
    pub github_personal_token: ProjectAccessToken,
    pub heroku_token: ProjectAccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatedGlobalAccessTokens {
    pub github_personal_token: ProjectAccessToken,
    pub heroku_token: Option<ProjectAccessToken>,
}

// Project configs
#[derive(Debug, Serialize, Deserialize)]
pub struct UnvalidatedProjectConfig {
    pub github_owner: String,
    pub github_repo: String,
    pub heroku_app: Option<String>,
    pub developers: u64,
    pub working_days_per_week: f32,
    pub access_tokens: UnvalidatedAccessTokens,
    pub deployment_source: DeploymentSource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatedGitHubDeploymentProjectConfig {
    pub github_owner: String,
    pub github_repo: String,
    pub heroku_app: Option<String>,
    pub developers: u64,
    pub working_days_per_week: f32,
    pub access_tokens: ValidatedGitHubDeploymentAccessTokens,
    pub deployment_source: DeploymentSource, // FIXME: デシリアライズ時に判定のために必要?消せる?
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatedHerokuReleasesProjectConfig {
    pub github_owner: String,
    pub github_repo: String,
    pub heroku_app: String,
    pub developers: u64,
    pub working_days_per_week: f32,
    pub access_tokens: ValidatedHerokuReleasesAccessTokens,
    pub deployment_source: DeploymentSource, // FIXME: デシリアライズ時に判定のために必要?消せる?
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValidatedProjectConfig {
    ValidatedGitHubDeploymentProjectConfig(ValidatedGitHubDeploymentProjectConfig),
    ValidatedHerokuReleasesProjectConfig(ValidatedHerokuReleasesProjectConfig),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigCreated {
    pub global: ValidatedGlobalAccessTokens,
    pub projects: HashMap<String, ValidatedProjectConfig>,
}

// Events
type CreateConfigEvent = ConfigCreated;

// Error types
#[derive(Debug, Clone)]
struct ConfigValidateError;

pub type CreateConfig = fn(UnvalidatedAccessTokens, UnvalidatedProjectConfig) -> Result<CreateConfigEvent, ConfigValidateError>;
