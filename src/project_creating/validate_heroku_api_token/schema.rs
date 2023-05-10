use thiserror::Error;

// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the ValidateHerokuApiToken workflow
// ==================================

// ------------------------------------
// inputs to the workflow
pub type UnvalidatedHerokuApiToken = Option<String>;

// ------------------------------------
// outputs from the workflow (success case)
#[derive(Clone)]
pub struct ValidatedHerokuApiToken(pub(super) String);

/// Event will be created if the Acknowledgment was successfully posted

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub type ValidateHerokuApiTokenEvent = ValidatedHerokuApiToken;

// Error types
// pub struct ValidateHerokuApiTokenError(pub(super) String);

#[derive(Debug, Error, Clone)]
pub enum ValidateHerokuApiTokenError {
    #[error("InvalidToken: {0}")]
    InvalidToken(String),
    #[error("InvalidToken: {0}")]
    Required(String),
}

// ------------------------------------
// the workflow itself
pub type ValidateHerokuApiToken =
    fn(
        UnvalidatedHerokuApiToken,
    ) -> Result<ValidateHerokuApiTokenEvent, ValidateHerokuApiTokenError>;
