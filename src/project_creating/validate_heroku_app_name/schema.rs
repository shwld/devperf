use thiserror::Error;

// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the ValidateHerokuAppName workflow
// ==================================

// ------------------------------------
// inputs to the workflow
pub type UnvalidatedHerokuAppName = Option<String>;

// ------------------------------------
// outputs from the workflow (success case)
#[derive(Clone)]
pub struct ValidatedHerokuAppName(pub(super) String);

/// Event will be created if the Acknowledgment was successfully posted

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub type ValidateHerokuAppNameEvent = ValidatedHerokuAppName;

// Error types
// pub struct ValidateHerokuAppNameError(pub(super) String);

#[derive(Debug, Error, Clone)]
pub enum ValidateHerokuAppNameError {
    #[error("InvalidName: {0}")]
    InvalidName(String),
    #[error("InvalidName: {0}")]
    Required(String),
}

// ------------------------------------
// the workflow itself
pub type ValidateHerokuAppName =
    fn(UnvalidatedHerokuAppName) -> Result<ValidateHerokuAppNameEvent, ValidateHerokuAppNameError>;
