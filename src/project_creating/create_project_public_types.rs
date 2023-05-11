// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the CreateProject workflow
// ==================================
use async_trait::async_trait;
use thiserror::Error;

use crate::{
    dependencies::project_config_io::writer::interface::ProjectConfigIOWriterError,
    project_parameter_validating::{
        validate_developer_count::ValidatedDeveloperCount,
        validate_github_owner_repo::ValidatedGitHubOwnerRepo,
        validate_github_personal_token::ValidatedGitHubPersonalToken,
        validate_heroku_app_name::ValidatedHerokuAppName,
        validate_heroku_auth_token::ValidatedHerokuAuthToken,
        validate_working_days_per_week::ValidatedWorkingDaysPerWeek,
    },
};

// ------------------------------------
// inputs to the workflow

// Project configs
pub struct UncreatedGitHubDeploymentProject {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
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
    HerokuRelease(UncreatedHerokuReleaseProject),
}

// ------------------------------------
// outputs from the workflow (success case)
#[derive(Clone)]
pub struct GitHubDeploymentProjectCreated {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
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
    WriteError(ProjectConfigIOWriterError),
}

// ------------------------------------
// the workflow itself
#[async_trait]
pub trait CreateProjectWorkflow {
    async fn create_project(
        &self,
        uncreated_project: UncreatedProject,
    ) -> Result<Vec<CreateProjectEvent>, CreateGithubDeploymentProjectError>;
}
