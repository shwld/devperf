use crate::{project_creating::{validate_github_personal_token::schema::*, validate_github_owner_repo::schema::*, schema::ProjectAccessToken}, common_types::{NonZeroU32, NonZeroF32, WriteConfigError}};

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
    pub developers: NonZeroU32,
    pub working_days_per_week: NonZeroF32,
}

// ------------------------------------
// outputs from the workflow (success case)
pub struct GitHubDeploymentProjectCreated {
    pub github_personal_token: ProjectAccessToken<ValidatedGitHubPersonalToken>,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub developers: NonZeroU32,
    pub working_days_per_week: NonZeroF32,
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
