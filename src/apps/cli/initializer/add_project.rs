use anyhow::Result;
use inquire::Select;

use crate::{
    apps::cli::initializer::github_pull_request, common_types::deployment_source::DeploymentSource,
};

use super::{github_deployment, heroku_release};

pub async fn perform() -> Result<()> {
    println!("Initialize CLI");
    let github_deployment = DeploymentSource::GitHubDeployment.label();
    let github_pull_request = DeploymentSource::GitHubPullRequest.label();
    let heroku_release = DeploymentSource::HerokuRelease.label();
    let options: Vec<&str> = vec![&github_deployment, &github_pull_request, &heroku_release];
    let answer = Select::new("Select Deployment Frequency Source: ", options).prompt()?;
    let source = DeploymentSource::try_new(answer).expect("Invalid deployment source");

    match source {
        DeploymentSource::GitHubDeployment => {
            github_deployment::add_project().await;
        }
        DeploymentSource::GitHubPullRequest => {
            github_pull_request::add_project().await;
        }
        DeploymentSource::HerokuRelease => {
            heroku_release::add_project().await;
        }
    }

    Ok(())
}
