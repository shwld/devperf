use inquire::{Password, PasswordDisplayMode};

use crate::common_types::heroku_auth_token::ValidatedHerokuAuthToken;

pub fn input() -> ValidatedHerokuAuthToken {
    let value = Password::new("Type a Heroku Authorization token: ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .unwrap();
    let value = ValidatedHerokuAuthToken::new(Some(value));

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid token");
        input()
    }
}

pub fn input_or_default(
    default_value: Option<ValidatedHerokuAuthToken>,
) -> ValidatedHerokuAuthToken {
    let value = Password::new("Type a Heroku Authorization token(if blank, use default): ")
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt()
        .unwrap();
    let value = ValidatedHerokuAuthToken::new(Some(value));

    if let Ok(value) = value {
        value
    } else if let Some(default_value) = default_value {
        default_value
    } else {
        input_or_default(None)
    }
}
