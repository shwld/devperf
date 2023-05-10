use inquire::Text;

use crate::project_creating::validate_github_owner_repo::{self, schema::ValidatedGitHubOwnerRepo};

pub fn input() -> ValidatedGitHubOwnerRepo {
    let value = Text::new("Type a GitHub owner/repo: ")
        .with_placeholder("owner/repo")
        .prompt()
        .unwrap();
    let value = validate_github_owner_repo::workflow::perform(value);

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid owner/repo");
        input()
    }
}
