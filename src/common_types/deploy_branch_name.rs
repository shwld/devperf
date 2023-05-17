use std::fmt;
use thiserror::Error;

#[derive(Clone)]
pub struct ValidatedDeployBranchName(pub(super) String);

#[derive(Debug, Error, Clone)]
pub enum ValidateDeployBranchNameError {
    #[error("InvalidName: {0}")]
    InvalidName(String),
    #[error("InvalidName: {0}")]
    Required(String),
}

impl ValidatedDeployBranchName {
    pub fn new(token: Option<String>) -> Result<Self, ValidateDeployBranchNameError> {
        if let Some(token) = token {
            if token.len() > 1 {
                Ok(ValidatedDeployBranchName(token))
            } else {
                Err(ValidateDeployBranchNameError::InvalidName(
                    "Heroku app name is invalid".to_string(),
                ))
            }
        } else {
            Err(ValidateDeployBranchNameError::Required(
                "Heroku app name is empty".to_string(),
            ))
        }
    }
}

impl fmt::Display for ValidatedDeployBranchName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
