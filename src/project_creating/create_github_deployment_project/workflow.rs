use crate::common_types::WriteConfigError;
use super::schema::*;

pub fn perform(write_project: WriteProject, project: UncreatedGitHubDeploymentProject) -> Result<GitHubDeploymentProjectCreated, WriteConfigError> {
    Ok(GitHubDeploymentProjectCreated {
        github_personal_token: project.github_personal_token,
        github_owner_repo: project.github_owner_repo,
        developer_count: project.developer_count,
        working_days_per_week: project.working_days_per_week,
    })
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
