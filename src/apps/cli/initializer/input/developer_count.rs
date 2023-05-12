use inquire::Text;

use crate::common_types::validate_developer_count::{self, ValidatedDeveloperCount};

pub fn input() -> ValidatedDeveloperCount {
    let value = Text::new("Type a Developer count: ")
        .with_placeholder("1")
        .prompt()
        .unwrap();
    let value = validate_developer_count::perform(value);

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid developer count");
        input()
    }
}
