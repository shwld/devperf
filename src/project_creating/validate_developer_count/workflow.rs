use super::schema::*;

pub fn perform(
    count: UnvalidatedDeveloperCount,
) -> Result<ValidateDeveloperCountEvent, ValidateDeveloperCountError> {
    ValidatedDeveloperCount::new(count)
}

// PRIVATE

impl ValidatedDeveloperCount {
    pub fn new(count: String) -> Result<Self, ValidateDeveloperCountError> {
        let count = count.parse::<u32>();
        if let Ok(count) = count {
            if count > 0 {
                Ok(ValidatedDeveloperCount(count))
            } else {
                Err(ValidateDeveloperCountError(
                    "Developer count is zero".to_string(),
                ))
            }
        } else {
            Err(ValidateDeveloperCountError(
                "Developer count is not a number".to_string(),
            ))
        }
    }

    pub fn to_u32(&self) -> u32 {
        self.0
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::project_creating::validate_developer_count::schema::ValidateDeveloperCount;

//     #[test]
//     fn verify_perform_type() {
//         // 型チェックのために代入する
//         let _type_check: ValidateDeveloperCount = super::perform;
//     }
// }
