use std::fmt;
use thiserror::Error;

#[derive(Clone)]
pub struct ValidatedGitHubPersonalToken(pub(super) String);

#[derive(Debug, Error, Clone)]
pub enum ValidateGitHubPersonalTokenError {
    #[error("InvalidToken: {0}")]
    InvalidToken(String),
    #[error("InvalidToken: {0}")]
    Required(String),
}

impl ValidatedGitHubPersonalToken {
    pub fn new(token: Option<String>) -> Result<Self, ValidateGitHubPersonalTokenError> {
        if let Some(token) = token {
            if token.starts_with("ghp_") {
                Ok(ValidatedGitHubPersonalToken(token))
            } else {
                Err(ValidateGitHubPersonalTokenError::InvalidToken(
                    "GitHub personal token is invalid".to_string(),
                ))
            }
        } else {
            Err(ValidateGitHubPersonalTokenError::Required(
                "GitHub personal token is empty".to_string(),
            ))
        }
    }
}

impl fmt::Display for ValidatedGitHubPersonalToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
