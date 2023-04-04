use confy;
use inquire::{Select};
use serde::{Serialize, Deserialize};
use tokio::task;
use core::panic;
use std::collections::HashMap;
use std::io::Write;
use rpassword::read_password;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub github_personal_token: Option<String>,
    pub heroku_token: Option<String>,
    pub projects: HashMap<String, ProjectConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub github_owner: String,
    pub github_repo: String,
    pub heroku_app: Option<String>,
    pub developers: u64,
    pub working_days_per_week: f32,
    pub github_personal_token: Option<String>,
    pub heroku_token: Option<String>,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self { Self {
        github_personal_token: None,
        heroku_token: None,
        projects: HashMap::new(),
    }}
}

pub async fn load_config() -> Config {
    let conf = task::spawn_blocking(|| confy::load::<Config>("devops-metrics-tools", None))
        .await
        .expect("Blocking task join error")
        .map(|c| {
            Config {
                github_personal_token: c.github_personal_token,
                heroku_token: c.heroku_token,
                projects: c.projects,
            }
        }).unwrap_or_default();

    conf
}

pub async fn store_config(config: Config) -> Result<Config, confy::ConfyError> {
    task::spawn_blocking(|| {
        confy::store("devops-metrics-tools", None, config.clone())?;
        Ok(config)
    })
        .await
        .expect("Blocking task join error")
}

pub fn get_config_path() -> Result<std::path::PathBuf, confy::ConfyError> {
    confy::get_configuration_file_path("devops-metrics-tools", None)
}

pub async fn set_github_personal_token() -> Result<(), confy::ConfyError> {
    print!("Type a GitHub Personal access token: ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let read_token = read_password().expect("Failed to read token");
    let token = read_token.is_empty().then(|| None).unwrap_or(Some(read_token));

    let mut config = load_config().await;
    config.github_personal_token = token;
    store_config(config).await?;

    Ok(())
}

pub async fn set_project_config(project_name: &str) -> Result<(), confy::ConfyError> {
    log::debug!("start: {:?}", project_name);
    let mut config = load_config().await;

    let options: Vec<&str> = vec!["GitHub deployments", "GitHub releases", "GitHub PullRequests", "Heroku deployments"];
    let ans = Select::new("Select Deployment Frequency Source: ", options).prompt().expect("Failed to prompt");
    let project_config: ProjectConfig = match ans {
        "GitHub deployments" => {
            print!("Type a GitHub Personal access token(Empty to use default): ");
            std::io::stdout().flush().expect("Failed to flush stdout");
            let read_token = read_password().expect("Failed to read token");
            let token = read_token.is_empty().then(|| None).unwrap_or(Some(read_token));

            print!("Type github owner/repo: ");
            std::io::stdout().flush().expect("Failed to flush stdout");
            let mut github_repo_input = String::new();
            std::io::stdin().read_line(&mut github_repo_input).expect("Failed to read line");
            let github_repo: &str = github_repo_input.trim();

            print!("Type a developer count: ");
            std::io::stdout().flush().expect("Failed to flush stdout");
            let mut developers_input = String::new();
            std::io::stdin().read_line(&mut developers_input).expect("Failed to read line");
            let developers = developers_input.is_empty().then(|| 1).unwrap_or(developers_input.trim().parse::<u64>().expect("Failed to parse u64"));

            print!("Type a working days per week: ");
            std::io::stdout().flush().expect("Failed to flush stdout");
            let mut working_days_per_week_input = String::new();
            std::io::stdin().read_line(&mut working_days_per_week_input).expect("Failed to read line");
            let working_days_per_week = working_days_per_week_input.is_empty().then(|| 1.0).unwrap_or(working_days_per_week_input.trim().parse::<f32>().expect("Failed to parse u64"));
            let owner = github_repo.split("/").nth(0).expect("Failed to get owner");
            let repo = github_repo.split("/").nth(1).expect("Failed to get owner");

            ProjectConfig {
                github_owner: owner.to_owned(),
                github_repo: repo.to_owned(),
                heroku_app: None,
                developers: developers,
                working_days_per_week: working_days_per_week,
                github_personal_token: token,
                heroku_token: None,
            }
        },
        _ => panic!("Not implemented"),
    };

    let updated_projects = config.projects.get_mut(project_name);
    match updated_projects {
        Some(p) => {
            log::debug!("p: {:?}", p);
            log::debug!("prj: {:?}", project_config);
            *p = project_config.clone();
        },
        None => {
            config.projects.insert(project_name.to_owned(), project_config.clone());
        }
    }
    let stored_config = store_config(config).await.expect("Failed to store config");

    println!("updated_config: {:?}", serde_json::to_string_pretty(&stored_config));

    Ok(())
}
