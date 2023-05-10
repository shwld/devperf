use inquire::{Password, PasswordDisplayMode};

use crate::project_creating::validate_heroku_api_token::{self, schema::ValidatedHerokuApiToken};

pub fn input() -> ValidatedHerokuApiToken {
    let value = Password::new("Type a Heroku Authorization token: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .unwrap();
    let value = validate_heroku_api_token::workflow::perform(Some(value));

    if value.is_ok() {
        value.unwrap()
    } else {
        println!("Invalid token");
        input()
    }
}
