use inquire::Text;

use crate::common_types::github_deployment_environment::ValidatedGitHubDeploymentEnvironment;

pub fn input() -> ValidatedGitHubDeploymentEnvironment {
    let value = Text::new("Type a GitHub Deployment environment name: ")
        .prompt()
        .unwrap();
    let value = ValidatedGitHubDeploymentEnvironment::new(Some(value));

    if let Ok(value) = value {
        value
    } else {
        println!("Invalid name, see https://docs.github.com/en/actions/deployment/targeting-different-environments/using-environments-for-deployment");
        input()
    }
}
