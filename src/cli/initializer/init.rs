use anyhow::Result;
use inquire::{Select};

use crate::{cli::initializer::{github_deployment}};

pub async fn perform() -> Result<()> {
    println!("Initialize CLI");
    let options: Vec<&str> = vec!["GitHub deployments", "GitHub releases", "GitHub PullRequests", "Heroku releases"];
    let answer = Select::new("Select Deployment Frequency Source: ", options).prompt()?;

    match answer {
        "GitHub deployments" => {
            github_deployment::init().await;
        },
        "Heroku releases" => {
            unimplemented!();
        },
        _ => panic!("Not implemented"),
    };

    Ok(())
}
