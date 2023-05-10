use thiserror::Error;

// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the ValidateGitHubPersonalToken workflow
// ==================================

// ------------------------------------
// inputs to the workflow
pub type UnvalidatedGitHubPersonalToken = Option<String>;

// ------------------------------------
// outputs from the workflow (success case)
#[derive(Clone)]
pub struct ValidatedGitHubPersonalToken(pub(super) String);

/// Event will be created if the Acknowledgment was successfully posted

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub type ValidateGitHubPersonalTokenEvent = ValidatedGitHubPersonalToken;

// Error types
// pub struct ValidateGitHubPersonalTokenError(pub(super) String);

#[derive(Debug, Error, Clone)]
pub enum ValidateGitHubPersonalTokenError {
    #[error("InvalidToken: {0}")]
    InvalidToken(String),
    #[error("InvalidToken: {0}")]
    Required(String),
}

// ------------------------------------
// the workflow itself
pub type ValidateGitHubPersonalToken =
    fn(
        UnvalidatedGitHubPersonalToken,
    ) -> Result<ValidateGitHubPersonalTokenEvent, ValidateGitHubPersonalTokenError>;
