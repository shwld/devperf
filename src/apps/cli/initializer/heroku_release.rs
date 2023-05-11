use crate::{
    dependencies::project_config_io::writer::settings_toml::ProjectConfigIOWriterWithSettingsToml,
    project_creating::{
        create_project_implementation::CreateProjectWorkflow,
        create_project_public_types::UncreatedHerokuReleaseProject,
        create_project_public_types::{CreateProject, UncreatedProject},
    },
};

use super::input::{
    developer_count, github_owner_repo, github_personal_token, heroku_app_name, heroku_auth_token,
    project_name, working_days_per_week,
};

pub async fn init() {
    let project_name = project_name::input();
    let heroku_app_name = heroku_app_name::input();
    let heroku_auth_token = heroku_auth_token::input();
    let github_token = github_personal_token::input();
    let owner_repo = github_owner_repo::input();
    let developer_count = developer_count::input();
    let working_days_per_week = working_days_per_week::input();

    let uncreated_project = UncreatedProject::HerokuRelease(UncreatedHerokuReleaseProject {
        project_name,
        github_owner_repo: owner_repo,
        heroku_app_name,
        heroku_auth_token,
        developer_count,
        working_days_per_week,
        github_personal_token: github_token,
    });

    let workflow = CreateProjectWorkflow {
        project_io_writer: ProjectConfigIOWriterWithSettingsToml,
    };

    match workflow.create_project(uncreated_project).await {
        Ok(_project) => {
            println!("Complete project creation!");
        }
        Err(err) => {
            println!("Failed to create project: {:?}", err);
        }
    }
}
