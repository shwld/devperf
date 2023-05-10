use inquire::{Password, PasswordDisplayMode};

use crate::project_parameter_validating::validate_heroku_auth_token::{
    self, ValidatedHerokuApiToken,
};

pub fn input() -> ValidatedHerokuApiToken {
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
