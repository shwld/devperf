use anyhow::anyhow;

use crate::common_types::deployment_source::DeploymentSource;
use crate::project_creating::dto::ProjectConfigDto;

use super::super::settings_toml::{Config, ProjectName};
use super::interface::{ProjectConfigIOReader, ProjectConfigIOReaderError};
use async_trait::async_trait;

pub struct ProjectConfigIOReaderWithSettingsToml;
#[async_trait]
impl ProjectConfigIOReader for ProjectConfigIOReaderWithSettingsToml {
    async fn read(
        &self,
        project_name: ProjectName,
    ) -> Result<ProjectConfigDto, ProjectConfigIOReaderError> {
        confy::load::<Config>("devops-metrics-tools", None)
            .map_err(|e| anyhow!(e))
            .map_err(ProjectConfigIOReaderError::ConfigFileReadError)
            .and_then(|c| {
                let project_config = c.projects.get(&project_name);
                if let Some(project_config) = project_config {
                    if project_config.clone().deployment_source.as_str()
                        == DeploymentSource::GitHubDeployment.value()
                    {
                        Ok(ProjectConfigDto {
                            project_name,
                            developer_count: project_config.clone().developer_count,
                            working_days_per_week: project_config.clone().working_days_per_week,
                            github_personal_token: project_config
                                .clone()
                                .github_personal_token
                                .unwrap_or(c.github_personal_token.clone()),
                            github_owner: project_config.clone().github_owner,
                            github_repo: project_config.clone().github_repo,
                            github_deployment_environment: project_config
                                .github_deployment_environment
                                .clone(),
                            heroku_app_name: None,
                            heroku_auth_token: None,
                            deployment_source: DeploymentSource::GitHubDeployment.value(),
                        })
                    } else if project_config.clone().deployment_source.as_str()
                        == DeploymentSource::HerokuRelease.value()
                    {
                        Ok(ProjectConfigDto {
                            project_name,
                            developer_count: project_config.clone().developer_count,
                            working_days_per_week: project_config.clone().working_days_per_week,
                            github_personal_token: project_config
                                .clone()
                                .github_personal_token
                                .unwrap_or(c.github_personal_token.clone()),
                            github_deployment_environment: None,
                            heroku_app_name: project_config.clone().heroku_app_name,
                            heroku_auth_token: project_config.clone().heroku_auth_token,
                            github_owner: project_config.clone().github_owner,
                            github_repo: project_config.clone().github_repo,
                            deployment_source: DeploymentSource::HerokuRelease.value(),
                        })
                    } else {
                        unimplemented!()
                    }
                } else {
                    Err(ProjectConfigIOReaderError::ProjectNotFound(
                        "Project not found".to_string(),
                    ))
                }
            })
    }
}
