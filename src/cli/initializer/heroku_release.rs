use crate::{project_creating::{create_heroku_release_project::{schema::{UncreatedHerokuReleaseProject}, self}}, dependencies::write_new_config::settings_toml::WriteNewConfigWithSettingsToml};
use super::{input::{github_personal_token, github_owner_repo, developer_count, working_days_per_week, project_name, heroku_app_name, heroku_api_token}};

pub async fn init() {
    let project_name = project_name::input();
    let heroku_app_name = heroku_app_name::input();
    let heroku_api_token = heroku_api_token::input();
    let github_token = github_personal_token::input();
    let owner_repo = github_owner_repo::input();
    let developer_count = developer_count::input();
    let working_days_per_week = working_days_per_week::input();

    let uncreated_project = UncreatedHerokuReleaseProject {
        project_name: project_name,
        github_owner_repo: owner_repo,
        heroku_app_name: heroku_app_name,
        heroku_api_token: heroku_api_token,
        developer_count: developer_count,
        working_days_per_week: working_days_per_week,
        github_personal_token: github_token,
    };

    match create_heroku_release_project::workflow::perform(WriteNewConfigWithSettingsToml, uncreated_project).await {
        Ok(_project) => {
            println!("Complete project creation!");
        },
        Err(err) => {
            println!("Failed to create project: {:?}", err);
        }
    }
}
