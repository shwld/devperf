// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the ValidateGitHubOwnerRepo workflow
// ==================================

// ------------------------------------
// inputs to the workflow
pub type UnvalidatedGitHubOwnerRepo = String;

// ------------------------------------
// outputs from the workflow (success case)
#[derive(Clone)]
pub struct ValidatedGitHubOwnerRepo {
    pub(super) github_owner: String,
    pub(super) github_repo: String,
}

/// Event will be created if the Acknowledgment was successfully posted

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub type ValidateGitHubOwnerRepoEvent = ValidatedGitHubOwnerRepo;

// Error types
#[derive(Debug, Clone)]
pub struct ValidateGitHubOwnerRepoError(pub(super) String);

// ------------------------------------
// the workflow itself
pub type ValidateGitHubOwnerRepo = fn(UnvalidatedGitHubOwnerRepo) -> Result<ValidateGitHubOwnerRepoEvent, ValidateGitHubOwnerRepoError>;
