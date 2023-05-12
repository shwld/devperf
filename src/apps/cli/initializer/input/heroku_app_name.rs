use inquire::Text;

use crate::common_types::heroku_app_name::ValidatedHerokuAppName;

pub fn input() -> ValidatedHerokuAppName {
    let value = Text::new("Type a Heroku app name: ").prompt().unwrap();
    let value = ValidatedHerokuAppName::new(Some(value));

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid token");
        input()
    }
}
