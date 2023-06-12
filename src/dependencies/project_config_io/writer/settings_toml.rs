use anyhow::anyhow;
use async_trait::async_trait;
use std::collections::HashMap;

use super::super::settings_toml::{Config, ProjectConfig};
use super::interface::{ProjectConfigIOWriter, ProjectConfigIOWriterError, WriteConfigData};
use crate::apps::cli::config::CONFY_APP_NAME;

#[derive(Debug, Clone)]
pub struct ProjectConfigIOWriterWithSettingsToml;
#[async_trait]
impl ProjectConfigIOWriter for ProjectConfigIOWriterWithSettingsToml {
    async fn write(&self, data: WriteConfigData) -> Result<(), ProjectConfigIOWriterError> {
        let config = confy::load::<Config>(CONFY_APP_NAME, None);
        let mut config = match config {
            Ok(c) => c,
            Err(_e) => Config {
                github_personal_token: data.github_personal_token.clone(),
                heroku_auth_token: data.heroku_auth_token.clone(),
                projects: HashMap::new(),
            },
        };

        let project_config = ProjectConfig {
            github_personal_token: if config.github_personal_token == data.github_personal_token {
                None
            } else {
                Some(data.github_personal_token)
            },
            github_owner: data.github_owner,
            github_repo: data.github_repo,
            github_deployment_environment: data.github_deployment_environment,
            github_deployment_branch_name: data.github_deployment_branch_name,
            heroku_app_name: data.heroku_app_name,
            heroku_auth_token: if config.heroku_auth_token == data.heroku_auth_token {
                None
            } else {
                data.heroku_auth_token
            },
            developer_count: data.developer_count,
            working_days_per_week: data.working_days_per_week,
            deployment_source: data.deployment_source,
        };

        *config
            .projects
            .entry(data.project_name)
            .or_insert(project_config) = project_config.clone();

        confy::store(CONFY_APP_NAME, None, config)
            .map_err(|e| anyhow!(e))
            .map_err(ProjectConfigIOWriterError::CannotWritten)
    }
}
