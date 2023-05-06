use std::collections::HashMap;

use super::interface::{WriteNewConfig, WriteNewConfigData, WriteNewConfigError, Config};


pub struct WriteNewConfigWithSettingsToml;
impl WriteNewConfig for WriteNewConfigWithSettingsToml {
    fn perform(&self, params: WriteNewConfigData) -> Result<(), WriteNewConfigError> {
        let mut config = Config {
            github_personal_token: params.github_personal_token,
            projects: HashMap::new(),
        };
        config.projects.insert(params.project_name, params.project_config);
        confy::store("devops-metrics-tools", None, config).map_err(|e| WriteNewConfigError::ConfigFileWriteError)
    }
}
