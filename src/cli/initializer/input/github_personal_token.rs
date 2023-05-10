use inquire::{Password, PasswordDisplayMode};

use crate::project_creating::validate_github_personal_token::{self, schema::ValidatedGitHubPersonalToken};

pub fn input() -> ValidatedGitHubPersonalToken {
  let value = Password::new("Type a GitHub Personal access token: ").with_display_mode(PasswordDisplayMode::Masked).without_confirmation().prompt().unwrap();
  let value = validate_github_personal_token::workflow::perform(Some(value));

  if value.is_ok() {
    return value.unwrap()
  } else {
    println!("Invalid token");
    return input()
  }
}
