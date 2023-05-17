use async_trait::async_trait;

use super::create_project::{
    CreateGithubDeploymentProjectError, CreateProjectEvent, GitHubDeploymentProjectCreated,
    GitHubPullRequestProjectCreated, HerokuReleaseProjectCreated, UncreatedGitHubDeploymentProject,
    UncreatedGitHubPullRequestProject, UncreatedHerokuReleaseProject, UncreatedProject,
};

// ---------------------------
// CreateStep
// ---------------------------
pub(super) type CreateGithubDeploymentProject =
    fn(uncreated_project: UncreatedGitHubDeploymentProject) -> GitHubDeploymentProjectCreated;

pub(super) type CreateGithubPullRequestProject =
    fn(uncreated_project: UncreatedGitHubPullRequestProject) -> GitHubPullRequestProjectCreated;

pub(super) type CreateHerokuProject =
    fn(uncreated_project: UncreatedHerokuReleaseProject) -> HerokuReleaseProjectCreated;

#[async_trait]
pub(super) trait CreateProjectStep {
    async fn create_project(
        &self,
        uncreated_project: UncreatedProject,
    ) -> Result<CreateProjectEvent, CreateGithubDeploymentProjectError>;
}

// ---------------------------
// Create events
// ---------------------------
pub(super) type CreateEvents = fn(project: CreateProjectEvent) -> Vec<CreateProjectEvent>;
