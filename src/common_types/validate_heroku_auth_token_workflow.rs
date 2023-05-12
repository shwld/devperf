use std::fmt;

use super::validate_heroku_auth_token_schema::*;

pub fn perform(
    token: UnvalidatedHerokuAuthToken,
) -> Result<ValidateHerokuAuthTokenEvent, ValidateHerokuAuthTokenError> {
    ValidatedHerokuAuthToken::new(token)
}

// PRIVATE

impl ValidatedHerokuAuthToken {
    pub fn new(token: Option<String>) -> Result<Self, ValidateHerokuAuthTokenError> {
        if let Some(token) = token {
            if token.len() > 20 {
                Ok(ValidatedHerokuAuthToken(token))
            } else {
                Err(ValidateHerokuAuthTokenError::InvalidToken(
                    "Heroku authorization token is invalid".to_string(),
                ))
            }
        } else {
            Err(ValidateHerokuAuthTokenError::Required(
                "Heroku authorization token is empty".to_string(),
            ))
        }
    }
}

impl fmt::Display for ValidatedHerokuAuthToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::project_creating::validate_heroku_auth_token::schema::ValidateHerokuAuthToken;

//     #[test]
//     fn verify_perform_type() {
//         // 型チェックのために代入する
//         let _type_check: ValidateHerokuAuthToken = super::perform;
//     }
// }
