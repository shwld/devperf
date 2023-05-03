use crate::{project_creating::{validate_github_personal_token::schema::*, validate_github_owner_repo::schema::*, schema::ProjectAccessToken, validate_developer_count::schema::ValidatedDeveloperCount, validate_working_days_per_week::schema::ValidatedWorkingDaysPerWeek}, common_types::{WriteConfigError}};

// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the CreateConfig workflow
// ==================================

// ------------------------------------
// inputs to the workflow

pub type WriteProject = fn () -> Result<(), WriteConfigError>;

// Project configs
pub struct UncreatedGitHubDeploymentProject {
    pub github_personal_token: ProjectAccessToken<ValidatedGitHubPersonalToken>,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}

// ------------------------------------
// outputs from the workflow (success case)
pub struct GitHubDeploymentProjectCreated {
    pub github_personal_token: ProjectAccessToken<ValidatedGitHubPersonalToken>,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub type CreateGithubDeploymentProjectEvent = GitHubDeploymentProjectCreated;

// Error types
pub type CreateGithubDeploymentProjectError = WriteConfigError;

// ------------------------------------
// the workflow itself
pub type CreateGithubDeploymentProject = fn (WriteProject, UncreatedGitHubDeploymentProject) -> Result<GitHubDeploymentProjectCreated, CreateGithubDeploymentProjectError>;
