use inquire::Text;

use crate::project_parameter_validating::validate_working_days_per_week::{
    self, ValidatedWorkingDaysPerWeek,
};

pub fn input() -> ValidatedWorkingDaysPerWeek {
    let value = Text::new("Type a Working days per weed: ")
        .with_placeholder("5.0")
        .prompt()
        .unwrap();
    let value = validate_working_days_per_week::perform(value);

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid working days per weed");
        input()
    }
}
