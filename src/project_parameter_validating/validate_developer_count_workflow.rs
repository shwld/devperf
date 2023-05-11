use super::validate_developer_count_schema::*;

pub fn perform(
    count: UnvalidatedDeveloperCount,
) -> Result<ValidateDeveloperCountEvent, ValidateDeveloperCountError> {
    ValidatedDeveloperCount::new(count)
}

// PRIVATE

impl ValidatedDeveloperCount {
    pub fn new(count: String) -> Result<Self, ValidateDeveloperCountError> {
        let count = count.parse::<u32>()?;
        if count > 0 {
            Ok(ValidatedDeveloperCount(count))
        } else {
            Err(ValidateDeveloperCountError::MustBeAPositiveInteger(
                "Developer count is zero".to_string(),
            ))
        }
    }

    pub fn to_u32(&self) -> u32 {
        self.0
    }
}
