use super::dto::GitHubDeploymentProjectCreatedDto;

pub type WriteGitHubDeploymentProjectCreated = fn (GitHubDeploymentProjectCreatedDto) -> Result<(), WriteGitHubDeploymentProjectCreatedError>;

// Error types
#[derive(Debug, Clone)]
pub struct WriteGitHubDeploymentProjectCreatedError(pub String);
