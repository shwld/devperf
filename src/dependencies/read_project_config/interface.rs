use async_trait::async_trait;
use thiserror::Error;

type ProjectName = String;

#[derive(Debug, Clone)]
pub enum DeploymentSource {
    GitHubDeployment,
    HerokuRelease,
}

#[derive(Debug, Clone)]
pub struct GitHubDeploymentResourceConfig {
    pub github_personal_token: String,
    pub github_owner: String,
    pub github_repo: String,
}

#[derive(Debug, Clone)]
pub enum ResourceConfig {
    GitHubDeployment(GitHubDeploymentResourceConfig),
}

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub project_name: ProjectName,
    pub developer_count: u32,
    pub working_days_per_week: f32,
    pub resource: ResourceConfig,
}

#[derive(Debug, Error)]
pub enum ReadProjectConfigError {
    #[error("Cannot read the config file")]
    ConfigFileReadError(#[source] anyhow::Error),
    #[error("Cannot parse the config file")]
    ConfigFileParseError(#[source] anyhow::Error),
    #[error("Cannot find the project")]
    ProjectNotFound(String),
}

#[async_trait]
pub trait ReadProjectConfig {
    async fn perform(&self, project_name: ProjectName) -> Result<ProjectConfig, ReadProjectConfigError>;
}
