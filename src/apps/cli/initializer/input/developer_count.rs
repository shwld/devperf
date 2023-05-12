use inquire::Text;

use crate::common_types::developer_count::ValidatedDeveloperCount;

pub fn input() -> ValidatedDeveloperCount {
    let value = Text::new("Type a Developer count: ")
        .with_placeholder("1")
        .prompt()
        .unwrap();
    let value = ValidatedDeveloperCount::new(value);

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid developer count");
        input()
    }
}
