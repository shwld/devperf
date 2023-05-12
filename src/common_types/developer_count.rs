use thiserror::Error;

#[derive(Clone)]
pub struct ValidatedDeveloperCount(pub(super) u32);

#[derive(Debug, Error)]
pub enum ValidateDeveloperCountError {
    #[error("Must be a positive integer")]
    MustBeAPositiveInteger(String),
    #[error("Must be a positive integer")]
    ParseError(#[from] std::num::ParseIntError),
}

impl ValidatedDeveloperCount {
    fn new(count: String) -> Result<ValidatedDeveloperCount, ValidateDeveloperCountError> {
        let count = count.parse::<u32>()?;
        if count > 0 {
            Ok(ValidatedDeveloperCount(count))
        } else {
            Err(ValidateDeveloperCountError::MustBeAPositiveInteger(
                "Developer count is zero".to_string(),
            ))
        }
    }

    fn to_u32(&self) -> u32 {
        self.0
    }
}
