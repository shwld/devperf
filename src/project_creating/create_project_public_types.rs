// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the CreateProject workflow
// ==================================
use async_trait::async_trait;
use thiserror::Error;

use crate::{
    common_types::{
        deploy_branch_name::ValidatedDeployBranchName, developer_count::ValidatedDeveloperCount,
        github_deployment_environment::ValidatedGitHubDeploymentEnvironment,
        github_owner_repo::ValidatedGitHubOwnerRepo,
        github_personal_token::ValidatedGitHubPersonalToken,
        heroku_app_name::ValidatedHerokuAppName, heroku_auth_token::ValidatedHerokuAuthToken,
        working_days_per_week::ValidatedWorkingDaysPerWeek,
    },
    dependencies::project_config_io::writer::interface::ProjectConfigIOWriterError,
};

// ------------------------------------
// inputs to the workflow

// Project configs
pub struct UncreatedGitHubDeploymentProject {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub github_deployment_environment: ValidatedGitHubDeploymentEnvironment,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}
pub struct UncreatedGitHubPullRequestProject {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub github_deploy_branch_name: ValidatedDeployBranchName,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}
pub struct UncreatedHerokuReleaseProject {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub heroku_app_name: ValidatedHerokuAppName,
    pub heroku_auth_token: ValidatedHerokuAuthToken,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}
pub enum UncreatedProject {
    GitHubDeployment(UncreatedGitHubDeploymentProject),
    GitHubPullRequest(UncreatedGitHubPullRequestProject),
    HerokuRelease(UncreatedHerokuReleaseProject),
}

// ------------------------------------
// outputs from the workflow (success case)
#[derive(Clone)]
pub struct GitHubDeploymentProjectCreated {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub github_deployment_environment: ValidatedGitHubDeploymentEnvironment,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}

#[derive(Clone)]
pub struct GitHubPullRequestProjectCreated {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub github_deploy_branch_name: ValidatedDeployBranchName,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}

#[derive(Clone)]
pub struct HerokuReleaseProjectCreated {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub heroku_app_name: ValidatedHerokuAppName,
    pub heroku_auth_token: ValidatedHerokuAuthToken,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}

#[derive(Clone)]
pub enum ProjectCreated {
    GitHubDeployment(GitHubDeploymentProjectCreated),
    GitHubPullRequest(GitHubPullRequestProjectCreated),
    HerokuRelease(HerokuReleaseProjectCreated),
}

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub enum CreateProjectEvent {
    ProjectCreated(ProjectCreated),
}

// Error types
#[derive(Error, Debug)]
pub enum CreateGithubDeploymentProjectError {
    #[error("Cannot write config")]
    WriteError(#[from] ProjectConfigIOWriterError),
}

// ------------------------------------
// the workflow itself
#[async_trait]
pub trait CreateProject {
    async fn create_project(
        self,
        uncreated_project: UncreatedProject,
    ) -> Result<Vec<CreateProjectEvent>, CreateGithubDeploymentProjectError>;
}
