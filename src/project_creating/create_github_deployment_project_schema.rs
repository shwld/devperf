use crate::{
    dependencies::write_new_config::interface::WriteNewConfigError,
    project_creating::{
        public_schema::GitHubDeploymentProjectConfig, validate_github_owner_repo::schema::*,
        validate_github_personal_token::schema::*,
        validate_working_days_per_week::schema::ValidatedWorkingDaysPerWeek,
    },
    project_parameter_validating::validate_developer_count::ValidatedDeveloperCount,
};
use thiserror::Error;

// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the CreateConfig workflow
// ==================================

// ------------------------------------
// inputs to the workflow

// Error types
#[derive(Debug, Clone)]
pub struct WriteGitHubDeploymentProjectCreatedError(pub String);

// Project configs
pub struct UncreatedGitHubDeploymentProject {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}

// ------------------------------------
// outputs from the workflow (success case)
pub type GitHubDeploymentProjectCreated = GitHubDeploymentProjectConfig;

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub type CreateGithubDeploymentProjectEvent = GitHubDeploymentProjectCreated;

// Error types
#[derive(Error, Debug)]
pub enum CreateGithubDeploymentProjectError {
    #[error("Cannot write the new config")]
    WriteNewConfigError(WriteNewConfigError),
}

// ------------------------------------
// the workflow itself
// pub type CreateGithubDeploymentProject =
//     fn(
//         dyn WriteNewConfig,
//         UncreatedGitHubDeploymentProject,
//     ) -> Result<GitHubDeploymentProjectCreated, CreateGithubDeploymentProjectError>;
