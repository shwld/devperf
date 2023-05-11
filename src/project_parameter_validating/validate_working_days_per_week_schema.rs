// ==================================
// This file contains the definitions of PUBLIC types (exposed at the boundary of the bounded context)
// related to the ValidateWorkingDaysPerWeek workflow
// ==================================

use thiserror::Error;

// ------------------------------------
// inputs to the workflow
pub type UnvalidatedWorkingDaysPerWeek = String;

// ------------------------------------
// outputs from the workflow (success case)
#[derive(Clone)]
pub struct ValidatedWorkingDaysPerWeek(pub(super) f32);

/// Event will be created if the Acknowledgment was successfully posted

// Events
/// The possible events resulting from the workflow
/// Not all events will occur, depending on the logic of the workflow
pub type ValidateWorkingDaysPerWeekEvent = ValidatedWorkingDaysPerWeek;

// Error types
#[derive(Debug, Error)]
pub enum ValidateWorkingDaysPerWeekError {
    #[error("Must be a positive integer")]
    Invalid(String),
    #[error("Must be a positive integer")]
    ParseError(#[from] std::num::ParseFloatError),
}

// ------------------------------------
// the workflow itself
// pub type ValidateWorkingDaysPerWeek =
//     fn(
//         UnvalidatedWorkingDaysPerWeek,
//     ) -> Result<ValidateWorkingDaysPerWeekEvent, ValidateWorkingDaysPerWeekError>;
