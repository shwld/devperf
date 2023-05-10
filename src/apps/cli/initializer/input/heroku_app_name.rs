use inquire::{Password, PasswordDisplayMode};

use crate::project_parameter_validating::validate_heroku_app_name::{self, ValidatedHerokuAppName};

pub fn input() -> ValidatedHerokuAppName {
    let value = Password::new("Type a Heroku app name: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .unwrap();
    let value = validate_heroku_app_name::perform(Some(value));

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid token");
        input()
    }
}
