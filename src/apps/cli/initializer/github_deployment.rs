use super::input::{
    developer_count, github_deployment_environment, github_owner_repo, github_personal_token,
    project_name, working_days_per_week,
};
use crate::{
    dependencies::project_config_io::{
        reader::{
            interface::ProjectConfigIOReader, settings_toml::ProjectConfigIOReaderWithSettingsToml,
        },
        writer::settings_toml::ProjectConfigIOWriterWithSettingsToml,
    },
    project_creating::create_project::{
        CreateProject, CreateProjectWorkflow, UncreatedGitHubDeploymentProject, UncreatedProject,
    },
};

pub async fn init() {
    let project_name = project_name::input();
    let environment = github_deployment_environment::input();
    let token = github_personal_token::input();
    let owner_repo = github_owner_repo::input();
    let developer_count = developer_count::input();
    let working_days_per_week = working_days_per_week::input();

    let uncreated_project = UncreatedProject::GitHubDeployment(UncreatedGitHubDeploymentProject {
        project_name,
        github_owner_repo: owner_repo,
        github_deployment_environment: environment,
        developer_count,
        working_days_per_week,
        github_personal_token: token,
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

pub async fn add_project() {
    let config = ProjectConfigIOReaderWithSettingsToml
        .read_globals()
        .await
        .expect("Failed to read project config");

    let project_name = project_name::input();
    let environment = github_deployment_environment::input();
    let token = github_personal_token::input_or_default(config.github_personal_token);
    let owner_repo = github_owner_repo::input();
    let developer_count = developer_count::input();
    let working_days_per_week = working_days_per_week::input();

    let uncreated_project = UncreatedProject::GitHubDeployment(UncreatedGitHubDeploymentProject {
        project_name,
        github_owner_repo: owner_repo,
        github_deployment_environment: environment,
        developer_count,
        working_days_per_week,
        github_personal_token: token,
    });
    let workflow = CreateProjectWorkflow {
        project_io_writer: ProjectConfigIOWriterWithSettingsToml,
    };

    match workflow.create_project(uncreated_project).await {
        Ok(_project) => println!("Complete project creation!"),
        Err(err) => println!("Failed to create project: {:?}", err),
    }
}
