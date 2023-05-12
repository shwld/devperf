use std::fmt;
use thiserror::Error;

#[derive(Clone)]
pub struct ValidatedGitHubDeploymentEnvironment(pub(super) String);

#[derive(Debug, Error, Clone)]
pub enum ValidateGitHubDeploymentEnvironmentError {
    #[error("Invalid: {0}")]
    Invalid(String),
    #[error("Required: {0}")]
    Required(String),
}

impl ValidatedGitHubDeploymentEnvironment {
    pub fn new(token: Option<String>) -> Result<Self, ValidateGitHubDeploymentEnvironmentError> {
        if let Some(token) = token {
            if !token.is_empty() {
                Ok(ValidatedGitHubDeploymentEnvironment(token))
            } else {
                Err(ValidateGitHubDeploymentEnvironmentError::Invalid(
                    "GitHub deployment environment name is invalid".to_string(),
                ))
            }
        } else {
            Err(ValidateGitHubDeploymentEnvironmentError::Required(
                "GitHub deployment environment name is empty".to_string(),
            ))
        }
    }
}

impl fmt::Display for ValidatedGitHubDeploymentEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
