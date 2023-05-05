use std::collections::HashMap;
use serde::Serialize;
use crate::project_creating::{create_github_deployment_project::{dto::{GitHubDeploymentProjectCreatedDto, GitHubDeploymentProjectConfigDto}, dao_interfaces::WriteGitHubDeploymentProjectCreatedError}};

pub type ProjectConfig = GitHubDeploymentProjectConfigDto;

#[derive(Debug, Clone, Serialize)]
pub struct Config {
    pub github_personal_token: String,
    pub projects: HashMap<String, ProjectConfig>,
}

pub fn write_new_config(project: GitHubDeploymentProjectCreatedDto) -> Result<(), WriteGitHubDeploymentProjectCreatedError> {
    let mut config = Config {
        github_personal_token: project.project_config.github_personal_token.clone(),
        projects: HashMap::new(),
    };
    config.projects.insert(project.project_name, project.project_config);
    confy::store("devops-metrics-tools", None, config).map_err(|e| WriteGitHubDeploymentProjectCreatedError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::project_creating::{create_github_deployment_project::dao_interfaces::WriteGitHubDeploymentProjectCreated};

    #[test]
    fn verify_perform_type() {
        // 型チェックのために代入する
        let _type_check: WriteGitHubDeploymentProjectCreated = super::write_new_config;
    }
}
