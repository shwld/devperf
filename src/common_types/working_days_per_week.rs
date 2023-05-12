use thiserror::Error;

#[derive(Clone)]
pub struct ValidatedWorkingDaysPerWeek(pub(super) f32);

#[derive(Debug, Error)]
pub enum ValidateWorkingDaysPerWeekError {
    #[error("Must be a positive integer")]
    Invalid(String),
    #[error("Must be a positive integer")]
    ParseError(#[from] std::num::ParseFloatError),
}

impl ValidatedWorkingDaysPerWeek {
    pub fn new(count: String) -> Result<Self, ValidateWorkingDaysPerWeekError> {
        let count = count.parse::<f32>()?;
        if count > 0.0 && count < 7.0 {
            Ok(ValidatedWorkingDaysPerWeek(count))
        } else {
            Err(ValidateWorkingDaysPerWeekError::Invalid(
                "Specify between 0.0 and 7.0".to_string(),
            ))
        }
    }

    pub fn to_f32(&self) -> f32 {
        self.0
    }
}
