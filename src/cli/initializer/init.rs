use inquire::{Select};

use crate::{cli::initializer::{github_deployment}};

pub fn perform() {
    println!("Initialize CLI");
    let options: Vec<&str> = vec!["GitHub deployments", "GitHub releases", "GitHub PullRequests", "Heroku releases"];
    let answer = Select::new("Select Deployment Frequency Source: ", options).prompt().unwrap();

    match answer {
        "GitHub deployments" => {
            github_deployment::init();
        },
        "Heroku releases" => {
            unimplemented!();
        },
        _ => panic!("Not implemented"),
    };
}
