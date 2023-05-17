use serde::{Deserialize, Serialize};

const GITHUB_DEPLOYMENT: &str = "git_hub_deployment";
const GITHUB_PULL_REQUEST: &str = "git_pull_request";
const HEROKU_RELEASE: &str = "heroku_release";

const DISPLAY_GITHUB_DEPLOYMENT: &str = "GitHub Deployment";
const DISPLAY_GITHUB_PULL_REQUEST: &str = "GitHub Pull Request";
const DISPLAY_HEROKU_RELEASE: &str = "Heroku Release";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentSource {
    GitHubDeployment,
    GitHubPullRequest,
    HerokuRelease,
}

impl DeploymentSource {
    pub fn try_new(value: &str) -> Result<Self, &str> {
        match value {
            GITHUB_DEPLOYMENT => Ok(DeploymentSource::GitHubDeployment),
            GITHUB_PULL_REQUEST => Ok(DeploymentSource::GitHubPullRequest),
            HEROKU_RELEASE => Ok(DeploymentSource::HerokuRelease),
            DISPLAY_GITHUB_DEPLOYMENT => Ok(DeploymentSource::GitHubDeployment),
            DISPLAY_GITHUB_PULL_REQUEST => Ok(DeploymentSource::GitHubPullRequest),
            DISPLAY_HEROKU_RELEASE => Ok(DeploymentSource::HerokuRelease),
            _ => Err("Invalid deployment source"),
        }
    }
    pub fn label(self) -> String {
        match self {
            DeploymentSource::GitHubDeployment => DISPLAY_GITHUB_DEPLOYMENT.to_string(),
            DeploymentSource::GitHubPullRequest => DISPLAY_GITHUB_PULL_REQUEST.to_string(),
            DeploymentSource::HerokuRelease => DISPLAY_HEROKU_RELEASE.to_string(),
        }
    }
    pub fn value(self) -> String {
        match self {
            DeploymentSource::GitHubDeployment => GITHUB_DEPLOYMENT.to_string(),
            DeploymentSource::GitHubPullRequest => GITHUB_PULL_REQUEST.to_string(),
            DeploymentSource::HerokuRelease => HEROKU_RELEASE.to_string(),
        }
    }
}
