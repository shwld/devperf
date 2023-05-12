use std::fmt;
use thiserror::Error;

#[derive(Clone)]
pub struct ValidatedHerokuAuthToken(pub(super) String);

#[derive(Debug, Error, Clone)]
pub enum ValidateHerokuAuthTokenError {
    #[error("InvalidToken: {0}")]
    InvalidToken(String),
    #[error("InvalidToken: {0}")]
    Required(String),
}

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
