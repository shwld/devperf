use regex::Regex;

use super::schema::*;

pub fn perform(owner_repo: UnvalidatedGitHubOwnerRepo) -> Result<ValidateGitHubOwnerRepoEvent, ValidateGitHubOwnerRepoError> {
    ValidatedGitHubOwnerRepo::new(owner_repo)
}

// PRIVATE

impl ValidatedGitHubOwnerRepo {
    pub fn new(owner_repo: String) -> Result<Self, ValidateGitHubOwnerRepoError> {
      let re = Regex::new(r"^([\w\d\-]+)\/([\w\d\-]+)$").unwrap();
      let caps = re.captures(&owner_repo);
      if caps.is_some() {
        let caps = caps.unwrap();
        Ok(ValidatedGitHubOwnerRepo {
          github_owner: caps.get(1).map_or("", |m| m.as_str()).to_string(),
          github_repo: caps.get(2).map_or("", |m| m.as_str()).to_string(),
        })
      } else {
        Err(ValidateGitHubOwnerRepoError("GitHub owner/repo is invalid".to_string()))
      }
    }

    pub fn get_owner(self) -> String {
      self.github_owner
    }

    pub fn get_repo(self) -> String {
      self.github_repo
    }
}

#[cfg(test)]
mod tests {
    use crate::project_creating::validate_github_owner_repo::schema::ValidateGitHubOwnerRepo;


    #[test]
    fn verify_perform_type() {
        // 型チェックのために代入する
        let _type_check: ValidateGitHubOwnerRepo = super::perform;
    }
}
