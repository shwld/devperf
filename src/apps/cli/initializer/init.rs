use anyhow::Result;
use inquire::Select;

use crate::common_types::deployment_source::DeploymentSource;

use super::{github_deployment, heroku_release};

pub async fn perform() -> Result<()> {
    println!("Initialize CLI");
    let github_deployment = DeploymentSource::GitHubDeployment.label();
    let github_pull_request = DeploymentSource::GitHubPullRequest.label();
    let heroku_release = DeploymentSource::HerokuRelease.label();
    let options: Vec<&str> = vec![&github_deployment, &github_pull_request, &heroku_release];
    let answer = Select::new("Select Deployment Frequency Source: ", options).prompt()?;

    if answer == github_deployment {
        github_deployment::init().await;
    } else if answer == heroku_release {
        heroku_release::init().await;
    } else {
        panic!("Not implemented");
    }

    Ok(())
}
