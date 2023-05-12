use regex::Regex;
use std::fmt;
use thiserror::Error;

#[derive(Clone)]
pub struct ValidatedGitHubOwnerRepo {
    pub(super) github_owner: String,
    pub(super) github_repo: String,
}

// Error types
#[derive(Debug, Error)]
pub enum ValidateGitHubOwnerRepoError {
    #[error("Owner repo is invalid")]
    Invalid(String),
}

impl ValidatedGitHubOwnerRepo {
    pub fn new(owner_repo: String) -> Result<Self, ValidateGitHubOwnerRepoError> {
        let re = Regex::new(r"^([\w\d\-]+)/([\w\d\-]+)$").unwrap();
        let caps = re.captures(&owner_repo);
        if let Some(caps) = caps {
            Ok(ValidatedGitHubOwnerRepo {
                github_owner: caps.get(1).map_or("", |m| m.as_str()).to_string(),
                github_repo: caps.get(2).map_or("", |m| m.as_str()).to_string(),
            })
        } else {
            Err(ValidateGitHubOwnerRepoError::Invalid(
                "GitHub owner/repo is invalid".to_string(),
            ))
        }
    }

    pub fn get_owner(&self) -> String {
        self.github_owner.clone()
    }

    pub fn get_repo(&self) -> String {
        self.github_repo.clone()
    }

    pub fn get_values(self) -> (String, String) {
        (self.github_owner, self.github_repo)
    }
}

impl fmt::Display for ValidatedGitHubOwnerRepo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.github_owner, self.github_repo)
    }
}
