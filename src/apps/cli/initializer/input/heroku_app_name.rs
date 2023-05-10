use inquire::{Password, PasswordDisplayMode};

use crate::project_creating::validate_heroku_app_name::{self, schema::ValidatedHerokuAppName};

pub fn input() -> ValidatedHerokuAppName {
    let value = Password::new("Type a Heroku app name: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .unwrap();
    let value = validate_heroku_app_name::workflow::perform(Some(value));

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid token");
        input()
    }
}
