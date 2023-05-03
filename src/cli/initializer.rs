use inquire::{Select, Text, Password, PasswordDisplayMode};

use crate::project_creating::{validate_github_personal_token, validate_github_owner_repo, validate_developer_count};

pub fn initialize_cli() {
    println!("Initialize CLI");
    let options: Vec<&str> = vec!["GitHub deployments", "GitHub releases", "GitHub PullRequests", "Heroku releases"];
    let answer = Select::new("Select Deployment Frequency Source: ", options).prompt().unwrap();

    let project_config = match answer {
        "GitHub deployments" => {
            let token = Password::new("Type a GitHub Personal access token: ").with_display_mode(PasswordDisplayMode::Masked).without_confirmation().prompt().unwrap();
            let token = validate_github_personal_token::workflow::perform(token).expect("Invalid token");
            println!("New token");
            let owner_repo = Text::new("Type a GitHub owner/repo: ").prompt().unwrap();
            let owner_repo = validate_github_owner_repo::workflow::perform(owner_repo).expect("Invalid token");

            let developer_count = Text::new("Type a Developer count: ").prompt().unwrap();
            let developer_count = validate_developer_count::workflow::perform(developer_count).expect("Invalid developer count");
        },
        "Heroku releases" => {
        },
        _ => panic!("Not implemented"),
    };
}
