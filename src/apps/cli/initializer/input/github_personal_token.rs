use inquire::{Password, PasswordDisplayMode};

use crate::common_types::validate_github_personal_token::{self, ValidatedGitHubPersonalToken};

pub fn input() -> ValidatedGitHubPersonalToken {
    let value = Password::new("Type a GitHub Personal access token: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .unwrap();
    let value = validate_github_personal_token::perform(Some(value));

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid token");
        input()
    }
}
