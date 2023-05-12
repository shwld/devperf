use std::fmt;

use regex::Regex;

use super::validate_github_owner_repo_schema::*;

pub fn perform(
    owner_repo: UnvalidatedGitHubOwnerRepo,
) -> Result<ValidateGitHubOwnerRepoEvent, ValidateGitHubOwnerRepoError> {
    ValidatedGitHubOwnerRepo::new(owner_repo)
}

// PRIVATE

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

// #[cfg(test)]
// mod tests {
//     use crate::project_creating::validate_github_owner_repo::schema::ValidateGitHubOwnerRepo;

//     #[test]
//     fn verify_perform_type() {
//         // 型チェックのために代入する
//         let _type_check: ValidateGitHubOwnerRepo = super::perform;
//     }
// }
