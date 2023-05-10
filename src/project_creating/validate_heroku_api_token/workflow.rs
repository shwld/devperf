use super::schema::*;

pub fn perform(
    token: UnvalidatedHerokuApiToken,
) -> Result<ValidateHerokuApiTokenEvent, ValidateHerokuApiTokenError> {
    ValidatedHerokuApiToken::new(token)
}

// PRIVATE

impl ValidatedHerokuApiToken {
    pub fn new(token: Option<String>) -> Result<Self, ValidateHerokuApiTokenError> {
        if let Some(token) = token {
            if token.len() > 20 {
                Ok(ValidatedHerokuApiToken(token))
            } else {
                Err(ValidateHerokuApiTokenError::InvalidToken(
                    "Heroku authorization token is invalid".to_string(),
                ))
            }
        } else {
            Err(ValidateHerokuApiTokenError::Required(
                "Heroku authorization token is empty".to_string(),
            ))
        }
    }

    pub fn to_string(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::project_creating::validate_heroku_api_token::schema::ValidateHerokuApiToken;

    #[test]
    fn verify_perform_type() {
        // 型チェックのために代入する
        let _type_check: ValidateHerokuApiToken = super::perform;
    }
}
