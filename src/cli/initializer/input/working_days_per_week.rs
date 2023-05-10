use inquire::Text;

use crate::project_creating::validate_working_days_per_week::{
    self, schema::ValidatedWorkingDaysPerWeek,
};

pub fn input() -> ValidatedWorkingDaysPerWeek {
    let value = Text::new("Type a Working days per weed: ")
        .with_placeholder("5.0")
        .prompt()
        .unwrap();
    let value = validate_working_days_per_week::workflow::perform(value);

    if value.is_ok() {
        value.unwrap()
    } else {
        println!("Invalid working days per weed");
        input()
    }
}
