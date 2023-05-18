use anyhow::anyhow;

use crate::common_types::deployment_source::DeploymentSource;
use crate::common_types::github_personal_token::ValidatedGitHubPersonalToken;
use crate::common_types::heroku_auth_token::ValidatedHerokuAuthToken;
use crate::project_creating::dto::ProjectConfigDto;

use super::super::settings_toml::{Config, ProjectName};
use super::interface::{GlobalConfig, ProjectConfigIOReader, ProjectConfigIOReaderError};
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
                let project_config = c.projects.get(&project_name).ok_or(
                    ProjectConfigIOReaderError::ProjectNotFound("Project not found".to_string()),
                )?;
                let source = DeploymentSource::try_new(&project_config.clone().deployment_source)
                    .map_err(|e| {
                    ProjectConfigIOReaderError::DataSourceIsInvalid(e.to_string())
                })?;
                match source {
                    DeploymentSource::GitHubDeployment => Ok(ProjectConfigDto {
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
                        github_deployment_branch_name: None,
                        heroku_app_name: None,
                        heroku_auth_token: None,
                        deployment_source: DeploymentSource::GitHubDeployment.value(),
                    }),
                    DeploymentSource::GitHubPullRequest => Ok(ProjectConfigDto {
                        project_name,
                        developer_count: project_config.clone().developer_count,
                        working_days_per_week: project_config.clone().working_days_per_week,
                        github_personal_token: project_config
                            .clone()
                            .github_personal_token
                            .unwrap_or(c.github_personal_token.clone()),
                        github_owner: project_config.clone().github_owner,
                        github_repo: project_config.clone().github_repo,
                        github_deployment_environment: None,
                        github_deployment_branch_name: project_config
                            .clone()
                            .github_deployment_branch_name,
                        heroku_app_name: None,
                        heroku_auth_token: None,
                        deployment_source: DeploymentSource::GitHubPullRequest.value(),
                    }),
                    DeploymentSource::HerokuRelease => Ok(ProjectConfigDto {
                        project_name,
                        developer_count: project_config.clone().developer_count,
                        working_days_per_week: project_config.clone().working_days_per_week,
                        github_personal_token: project_config
                            .clone()
                            .github_personal_token
                            .unwrap_or(c.github_personal_token.clone()),
                        github_deployment_environment: None,
                        github_deployment_branch_name: None,
                        heroku_app_name: project_config.clone().heroku_app_name,
                        heroku_auth_token: match project_config.clone().heroku_auth_token {
                            Some(token) => Some(token),
                            None => c.heroku_auth_token.clone(),
                        },
                        github_owner: project_config.clone().github_owner,
                        github_repo: project_config.clone().github_repo,
                        deployment_source: DeploymentSource::HerokuRelease.value(),
                    }),
                }
            })
    }

    async fn read_globals(&self) -> Result<GlobalConfig, ProjectConfigIOReaderError> {
        confy::load::<Config>("devops-metrics-tools", None)
            .map_err(|e| anyhow!(e))
            .map_err(ProjectConfigIOReaderError::ConfigFileReadError)
            .and_then(|c| {
                let heroku_auth_token = match c.heroku_auth_token {
                    Some(token) => {
                        Some(ValidatedHerokuAuthToken::new(Some(token)).map_err(|e| {
                            ProjectConfigIOReaderError::DataSourceIsInvalid(e.to_string())
                        })?)
                    }
                    None => None,
                };
                Ok(GlobalConfig {
                    github_personal_token: ValidatedGitHubPersonalToken::new(Some(
                        c.github_personal_token,
                    ))
                    .map_err(|e| ProjectConfigIOReaderError::DataSourceIsInvalid(e.to_string()))?,
                    heroku_auth_token,
                })
            })
    }
}
