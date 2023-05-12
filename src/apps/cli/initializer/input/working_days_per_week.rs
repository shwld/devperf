use inquire::Text;

use crate::common_types::working_days_per_week::ValidatedWorkingDaysPerWeek;

pub fn input() -> ValidatedWorkingDaysPerWeek {
    let value = Text::new("Type a Working days per weed: ")
        .with_placeholder("5.0")
        .prompt()
        .unwrap();
    let value = ValidatedWorkingDaysPerWeek::new(value);

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid working days per weed");
        input()
    }
}
