use crate::{
    dependencies::write_new_config::interface::WriteNewConfigError,
    project_creating::{
        public_schema::HerokuReleaseProjectConfig, validate_github_personal_token::schema::*,
        validate_heroku_api_token::schema::ValidatedHerokuApiToken,
        validate_heroku_app_name::schema::ValidatedHerokuAppName,
        validate_working_days_per_week::schema::ValidatedWorkingDaysPerWeek,
    },
    project_parameter_validating::{
        validate_developer_count::ValidatedDeveloperCount,
        validate_github_owner_repo::ValidatedGitHubOwnerRepo,
    },
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
pub struct WriteHerokuReleaseProjectCreatedError(pub String);

// Project configs
pub struct UncreatedHerokuReleaseProject {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub heroku_app_name: ValidatedHerokuAppName,
    pub heroku_api_token: ValidatedHerokuApiToken,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}

// ------------------------------------
// outputs from the workflow (success case)
pub type HerokuReleaseProjectCreated = HerokuReleaseProjectConfig;

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub type CreateHerokuReleaseProjectEvent = HerokuReleaseProjectCreated;

// Error types
#[derive(Error, Debug)]
pub enum CreateHerokuReleaseProjectError {
    #[error("Cannot write the new config")]
    WriteNewConfigError(WriteNewConfigError),
}

// ------------------------------------
// the workflow itself
// pub type CreateHerokuReleaseProject =
//     fn(
//         dyn WriteNewConfig,
//         UncreatedHerokuReleaseProject,
//     ) -> Result<HerokuReleaseProjectCreated, CreateHerokuReleaseProjectError>;
