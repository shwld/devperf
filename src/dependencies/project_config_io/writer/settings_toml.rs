use anyhow::anyhow;
use std::collections::HashMap;

use async_trait::async_trait;

use super::super::settings_toml::{Config, ProjectConfig};

use super::interface::{ProjectConfigIOWriter, ProjectConfigIOWriterError, WriteConfigData};

#[derive(Debug, Clone)]
pub struct ProjectConfigIOWriterWithSettingsToml;
#[async_trait]
impl ProjectConfigIOWriter for ProjectConfigIOWriterWithSettingsToml {
    async fn write(&self, data: WriteConfigData) -> Result<(), ProjectConfigIOWriterError> {
        let config = confy::load::<Config>("devops-metrics-tools", None);
        let mut config = match config {
            Ok(c) => c,
            Err(_e) => Config {
                github_personal_token: data.github_personal_token.clone(),
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
            heroku_app_name: data.heroku_app_name,
            heroku_auth_token: data.heroku_auth_token,
            developer_count: data.developer_count,
            working_days_per_week: data.working_days_per_week,
            deployment_source: data.deployment_source,
        };

        config
            .projects
            .entry(data.project_name)
            .or_insert(project_config);

        confy::store("devops-metrics-tools", None, config)
            .map_err(|e| anyhow!(e))
            .map_err(ProjectConfigIOWriterError::CannotWritten)
    }
}
