use inquire::{Password, PasswordDisplayMode};

use crate::common_types::validate_heroku_auth_token::{self, ValidatedHerokuAuthToken};

pub fn input() -> ValidatedHerokuAuthToken {
    let value = Password::new("Type a Heroku Authorization token: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .unwrap();
    let value = validate_heroku_auth_token::perform(Some(value));

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid token");
        input()
    }
}
