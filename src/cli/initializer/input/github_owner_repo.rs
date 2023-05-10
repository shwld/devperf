use inquire::{Text};

use crate::project_creating::{validate_github_owner_repo::{self, schema::ValidatedGitHubOwnerRepo}};

pub fn input() -> ValidatedGitHubOwnerRepo {
  let value = Text::new("Type a GitHub owner/repo: ").with_placeholder("owner/repo").prompt().unwrap();
  let value = validate_github_owner_repo::workflow::perform(value);

  if value.is_ok() {
    return value.unwrap()
  } else {
    println!("Invalid owner/repo");
    return input()
  }
}
