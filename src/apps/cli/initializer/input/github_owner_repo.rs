use inquire::Text;

use crate::common_types::github_owner_repo::ValidatedGitHubOwnerRepo;

pub fn input() -> ValidatedGitHubOwnerRepo {
    let value = Text::new("Type a GitHub owner/repo: ")
        .with_placeholder("owner/repo")
        .prompt()
        .unwrap();
    let value = ValidatedGitHubOwnerRepo::new(value);

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid owner/repo");
        input()
    }
}
