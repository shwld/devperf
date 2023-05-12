use inquire::Text;

use crate::common_types::validate_github_owner_repo::{self, ValidatedGitHubOwnerRepo};

pub fn input() -> ValidatedGitHubOwnerRepo {
    let value = Text::new("Type a GitHub owner/repo: ")
        .with_placeholder("owner/repo")
        .prompt()
        .unwrap();
    let value = validate_github_owner_repo::perform(value);

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid owner/repo");
        input()
    }
}
