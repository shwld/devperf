use inquire::{Text};

use crate::project_creating::{validate_developer_count::{self, schema::ValidatedDeveloperCount}};

pub fn input() -> ValidatedDeveloperCount {
  let value = Text::new("Type a Developer count: ").with_placeholder("1").prompt().unwrap();
  let value = validate_developer_count::workflow::perform(value);

  if value.is_ok() {
    return value.unwrap()
  } else {
    println!("Invalid developer count");
    return input()
  }
}
