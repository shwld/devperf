use std::collections::HashMap;

use async_trait::async_trait;

use crate::{dependencies::settings_toml::Config};

use super::interface::{WriteNewConfig, WriteNewConfigError, WriteConfigData};


pub struct WriteNewConfigWithSettingsToml;
#[async_trait]
impl WriteNewConfig for WriteNewConfigWithSettingsToml {
    async fn perform(&self, params: WriteConfigData) -> Result<(), WriteNewConfigError> {
        let mut config = Config {
            github_personal_token: params.github_personal_token,
            projects: HashMap::new(),
        };
        config.projects.insert(params.project_name, params.project_config);
        confy::store("devops-metrics-tools", None, config).map_err(|e| WriteNewConfigError::ConfigFileWriteError)
    }
}
