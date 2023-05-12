// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the ValidateDeveloperCount workflow
// ==================================

use thiserror::Error;

// ------------------------------------
// inputs to the workflow
pub type UnvalidatedDeveloperCount = String;

// ------------------------------------
// outputs from the workflow (success case)
#[derive(Clone)]
pub struct ValidatedDeveloperCount(pub(super) u32);

/// Event will be created if the Acknowledgment was successfully posted

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub type ValidateDeveloperCountEvent = ValidatedDeveloperCount;

// Error types
#[derive(Debug, Error)]
pub enum ValidateDeveloperCountError {
    #[error("Must be a positive integer")]
    MustBeAPositiveInteger(String),
    #[error("Must be a positive integer")]
    ParseError(#[from] std::num::ParseIntError),
}

// ------------------------------------
// the workflow itself
// pub type ValidateDeveloperCount =
//     fn(
//         UnvalidatedDeveloperCount,
//     ) -> Result<ValidateDeveloperCountEvent, ValidateDeveloperCountError>;
