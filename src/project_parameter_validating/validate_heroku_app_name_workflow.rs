use std::fmt;

use super::validate_heroku_app_name_schema::*;

pub fn perform(
    token: UnvalidatedHerokuAppName,
) -> Result<ValidateHerokuAppNameEvent, ValidateHerokuAppNameError> {
    ValidatedHerokuAppName::new(token)
}

// PRIVATE

impl ValidatedHerokuAppName {
    pub fn new(token: Option<String>) -> Result<Self, ValidateHerokuAppNameError> {
        if let Some(token) = token {
            if token.len() > 1 {
                Ok(ValidatedHerokuAppName(token))
            } else {
                Err(ValidateHerokuAppNameError::InvalidName(
                    "Heroku app name is invalid".to_string(),
                ))
            }
        } else {
            Err(ValidateHerokuAppNameError::Required(
                "Heroku app name is empty".to_string(),
            ))
        }
    }
}

impl fmt::Display for ValidatedHerokuAppName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::project_creating::validate_heroku_app_name::schema::ValidateHerokuAppName;

//     #[test]
//     fn verify_perform_type() {
//         // 型チェックのために代入する
//         let _type_check: ValidateHerokuAppName = super::perform;
//     }
// }
