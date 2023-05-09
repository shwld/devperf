use std::collections::HashMap;
use anyhow::anyhow;

use async_trait::async_trait;
use crate::{dependencies::settings_toml::{ProjectName, Config}};
use super::interface::{ReadProjectConfig, ReadProjectConfigError, ProjectConfig, DeploymentSource, GitHubDeploymentResourceConfig, ResourceConfig};

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self { Self {
        github_personal_token: "".to_string(),
        projects: HashMap::new(),
    }}
}

pub struct ReadProjectConfigWithSettingsToml;
#[async_trait]
impl ReadProjectConfig for ReadProjectConfigWithSettingsToml {
    async fn perform(&self, project_name: ProjectName) -> Result<ProjectConfig, ReadProjectConfigError> {
        let conf = confy::load::<Config>("devops-metrics-tools", None)
                .map_err(|e| anyhow!(e))
                .map_err(ReadProjectConfigError::ConfigFileReadError)
                .and_then(|c| {
                    let project_config = c.projects.get(&project_name);
                    if let Some(project_config) = project_config {
                        match project_config.clone().deployment_source.as_str() {
                            "github_deployment" => {
                                Ok(ProjectConfig {
                                    project_name: project_name,
                                    developer_count: project_config.clone().developer_count,
                                    working_days_per_week: project_config.clone().working_days_per_week,
                                    resource: ResourceConfig::GitHubDeployment(GitHubDeploymentResourceConfig {
                                        github_personal_token: project_config.clone().github_personal_token.unwrap_or(c.github_personal_token.clone()),
                                        github_owner: project_config.clone().github_owner,
                                        github_repo: project_config.clone().github_repo,
                                    }),
                                })
                            },
                            _ => unimplemented!(),
                        }
                    } else {
                        Err(ReadProjectConfigError::ProjectNotFound("Project not found".to_string()))
                    }
                });

        conf
    }
}
