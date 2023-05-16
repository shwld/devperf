use inquire::Text;

use crate::common_types::deploy_branch_name::ValidatedDeployBranchName;

pub fn input() -> ValidatedDeployBranchName {
    let value = Text::new("Type a Deploy branch name: ")
        .with_placeholder("main")
        .prompt()
        .unwrap();
    let value = ValidatedDeployBranchName::new(Some(value));

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid deploy branch name");
        input()
    }
}
