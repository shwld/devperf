use crate::dependencies::write_new_config::interface::WriteNewConfig;

use super::dto::{GitHubDeploymentProjectCreatedDto};
use super::schema::*;

pub async fn perform<T: WriteNewConfig>(write_new_config: T, project: UncreatedGitHubDeploymentProject) -> Result<CreateGithubDeploymentProjectEvent, CreateGithubDeploymentProjectError> {
    let project = GitHubDeploymentProjectCreated {
        project_name: project.project_name,
        github_personal_token: project.github_personal_token,
        github_owner_repo: project.github_owner_repo,
        developer_count: project.developer_count,
        working_days_per_week: project.working_days_per_week,
    };
    let project_dto = GitHubDeploymentProjectCreatedDto::from_git_hub_deployment_project_created(project.clone());
    write_new_config.perform(project_dto).await.map_err(CreateGithubDeploymentProjectError::WriteNewConfigError)?;
    Ok(project)
}
