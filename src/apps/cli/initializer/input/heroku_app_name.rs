use inquire::{Password, PasswordDisplayMode};

use crate::common_types::heroku_app_name::ValidatedHerokuAppName;

pub fn input() -> ValidatedHerokuAppName {
    let value = Password::new("Type a Heroku app name: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .unwrap();
    let value = ValidatedHerokuAppName::new(Some(value));

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid token");
        input()
    }
}
