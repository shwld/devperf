use std::fmt;
use thiserror::Error;

#[derive(Clone)]
pub struct ValidatedHerokuAppName(pub(super) String);

#[derive(Debug, Error, Clone)]
pub enum ValidateHerokuAppNameError {
    #[error("InvalidName: {0}")]
    InvalidName(String),
    #[error("InvalidName: {0}")]
    Required(String),
}

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
