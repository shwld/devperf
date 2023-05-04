use crate::project_creating::{create_github_deployment_project::{schema::UncreatedGitHubDeploymentProject, self}, schema::ProjectAccessToken};

use super::input::{github_personal_token, github_owner_repo, developer_count, working_days_per_week};

pub fn init() {
    let token = github_personal_token::input();
    let owner_repo = github_owner_repo::input();
    let developer_count = developer_count::input();
    let working_days_per_week = working_days_per_week::input();

    let uncreated_project = UncreatedGitHubDeploymentProject {
        github_owner_repo: owner_repo,
        developer_count: developer_count,
        working_days_per_week: working_days_per_week,
        github_personal_token: ProjectAccessToken::Override(token),
    };

    let res = create_github_deployment_project::workflow::perform(|| Ok(()), uncreated_project);
}
