use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentSource {
    GitHubDeployment,
    HerokuRelease,
}

impl DeploymentSource {
    pub fn label(self) -> String {
        match self {
            DeploymentSource::GitHubDeployment => "GitHub Deployment".to_string(),
            DeploymentSource::HerokuRelease => "Heroku Release".to_string(),
        }
    }
    pub fn value(self) -> String {
        match self {
            DeploymentSource::GitHubDeployment => "git_hub_deployment".to_string(),
            DeploymentSource::HerokuRelease => "heroku_release".to_string(),
        }
    }
}
