use inquire::{Select, Text, Password, PasswordDisplayMode};

use crate::project_creating::{validate_github_personal_token, validate_github_owner_repo, validate_developer_count, create_github_deployment_project::{self, schema::UncreatedGitHubDeploymentProject}, validate_working_days_per_week, schema::ProjectAccessToken};

pub fn initialize_cli() {
    println!("Initialize CLI");
    let options: Vec<&str> = vec!["GitHub deployments", "GitHub releases", "GitHub PullRequests", "Heroku releases"];
    let answer = Select::new("Select Deployment Frequency Source: ", options).prompt().unwrap();

    let project_config = match answer {
        "GitHub deployments" => {
            let token = Password::new("Type a GitHub Personal access token: ").with_display_mode(PasswordDisplayMode::Masked).without_confirmation().prompt().unwrap();
            let token = validate_github_personal_token::workflow::perform(token).expect("Invalid token");

            let owner_repo = Text::new("Type a GitHub owner/repo: ").prompt().unwrap();
            let owner_repo = validate_github_owner_repo::workflow::perform(owner_repo).expect("Invalid token");

            let developer_count = Text::new("Type a Developer count(1~): ").prompt().unwrap();
            let developer_count = validate_developer_count::workflow::perform(developer_count).expect("Invalid developer count");

            let working_days_per_week = Text::new("Type a Working days per week(1.0~7.0): ").prompt().unwrap();
            let working_days_per_week = validate_working_days_per_week::workflow::perform(working_days_per_week).expect("Invalid working days per week");

            let uncreated_project = UncreatedGitHubDeploymentProject {
                github_owner_repo: owner_repo,
                developer_count: developer_count,
                working_days_per_week: working_days_per_week,
                github_personal_token: ProjectAccessToken::Override(token),
            };

            create_github_deployment_project::workflow::perform(|| Ok(()), uncreated_project)
        },
        "Heroku releases" => {
            unimplemented!();
        },
        _ => panic!("Not implemented"),
    };
}
