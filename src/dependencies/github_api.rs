use octocrab::Octocrab;
use thiserror::Error;

use crate::project_parameter_validating::validate_github_personal_token::ValidatedGitHubPersonalToken;

#[derive(Clone)]
pub struct GitHubAPI {
    pub octocrab: Octocrab,
}
#[derive(Error, Debug)]
pub enum GitHubClientError {
    #[error("Octocrab error")]
    OctocrabError(#[from] octocrab::Error),
}

impl GitHubAPI {
    pub fn new(
        self,
        github_personal_token: ValidatedGitHubPersonalToken,
    ) -> Result<Self, GitHubClientError> {
        let octocrab = Octocrab::builder()
            .personal_token(github_personal_token.to_string())
            .build()?;

        Ok(Self { octocrab })
    }

    pub fn get_client(&self) -> Octocrab {
        self.octocrab
    }
}
