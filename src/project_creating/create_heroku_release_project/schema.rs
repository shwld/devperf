use crate::{project_creating::{validate_github_personal_token::schema::*, validate_github_owner_repo::schema::*, validate_developer_count::schema::ValidatedDeveloperCount, validate_working_days_per_week::schema::ValidatedWorkingDaysPerWeek, public_schema::HerokuReleaseProjectConfig, validate_heroku_app_name::schema::ValidatedHerokuAppName, validate_heroku_api_token::schema::ValidatedHerokuApiToken}, dependencies::write_new_config::interface::{WriteNewConfigError, WriteNewConfig}};
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
    WriteNewConfigError(WriteNewConfigError)
}

// ------------------------------------
// the workflow itself
pub type CreateHerokuReleaseProject = fn (dyn WriteNewConfig, UncreatedHerokuReleaseProject) -> Result<HerokuReleaseProjectCreated, CreateHerokuReleaseProjectError>;