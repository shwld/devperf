use super::input::{
    developer_count, github_owner_repo, github_personal_token, project_name, working_days_per_week,
};
use crate::{
    dependencies::write_new_config::settings_toml::WriteNewConfigWithSettingsToml,
    project_creating::create_github_deployment_project::{
        self, schema::UncreatedGitHubDeploymentProject,
    },
};

pub async fn init() {
    let project_name = project_name::input();
    let token = github_personal_token::input();
    let owner_repo = github_owner_repo::input();
    let developer_count = developer_count::input();
    let working_days_per_week = working_days_per_week::input();

    let uncreated_project = UncreatedGitHubDeploymentProject {
        project_name,
        github_owner_repo: owner_repo,
        developer_count,
        working_days_per_week,
        github_personal_token: token,
    };

    match create_github_deployment_project::workflow::perform(
        WriteNewConfigWithSettingsToml,
        uncreated_project,
    )
    .await
    {
        Ok(_project) => {
            println!("Complete project creation!");
        }
        Err(err) => {
            println!("Failed to create project: {:?}", err);
        }
    }
}
