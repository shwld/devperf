use super::input::{
    developer_count, github_deploy_branch_name, github_owner_repo, github_personal_token,
    project_name, working_days_per_week,
};
use crate::{
    dependencies::project_config_io::writer::settings_toml::ProjectConfigIOWriterWithSettingsToml,
    project_creating::create_project::{
        CreateProject, CreateProjectWorkflow, UncreatedGitHubPullRequest, UncreatedProject,
    },
};

pub async fn init() {
    let project_name = project_name::input();
    let branch = github_deploy_branch_name::input();
    let token = github_personal_token::input();
    let owner_repo = github_owner_repo::input();
    let developer_count = developer_count::input();
    let working_days_per_week = working_days_per_week::input();

    let uncreated_project = UncreatedProject::GitHubPullRequest(UncreatedGitHubPullRequest {
        project_name,
        github_owner_repo: owner_repo,
        github_deploy_branch_name: branch,
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
