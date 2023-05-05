use super::dao_interfaces::{WriteGitHubDeploymentProjectCreated};
use super::dto::{GitHubDeploymentProjectCreatedDto};
use super::schema::*;

pub fn perform(write_project: WriteGitHubDeploymentProjectCreated, project: UncreatedGitHubDeploymentProject) -> Result<CreateGithubDeploymentProjectEvent, CreateGithubDeploymentProjectError> {
    let project = GitHubDeploymentProjectCreated {
        project_name: project.project_name,
        github_personal_token: project.github_personal_token,
        github_owner_repo: project.github_owner_repo,
        developer_count: project.developer_count,
        working_days_per_week: project.working_days_per_week,
    };
    let project_dto = GitHubDeploymentProjectCreatedDto::from_git_hub_deployment_project_created(project.clone());
    write_project(project_dto)?;
    Ok(project)
}

#[cfg(test)]
mod tests {
    use crate::project_creating::create_github_deployment_project::schema::CreateGithubDeploymentProject;

    #[test]
    fn verify_perform_type() {
        // 型チェックのために代入する
        let _type_check: CreateGithubDeploymentProject = super::perform;
    }
}
