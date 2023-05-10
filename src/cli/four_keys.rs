use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{
    dependencies::{
        fetch_deployments::{
            github_deployment::FetchDeploymentsWithGithubDeployment,
            heroku_release::FetchDeploymentsWithHerokuRelease,
        },
        get_first_commit_from_compare::github::GetFirstCommitFromCompareWithGitHub,
        github_api::GitHubAPI,
        read_project_config::{
            interface::ReadProjectConfig, settings_toml::ReadProjectConfigWithSettingsToml,
        },
    },
    metrics_retrieving::retrieve_four_keys::{
        self, FourKeysMetrics, RetrieveFourKeysExecutionContext,
    },
};

pub async fn get_four_keys(
    project_name: &str,
    since: DateTime<Utc>,
    until: DateTime<Utc>,
    environment: &str,
) -> Result<FourKeysMetrics> {
    let read_config = ReadProjectConfigWithSettingsToml;
    let project_config = read_config.perform(project_name.to_string()).await?;
    let github_api = GitHubAPI {
        project_config: project_config.clone(),
    };
    let fetch_deployments_with_github_deployment = FetchDeploymentsWithGithubDeployment {
        github_api: github_api.clone(),
    };
    let fetch_deployments_with_heroku_release = FetchDeploymentsWithHerokuRelease {
        project_config: project_config.clone(),
        github_api: github_api.clone(),
    };
    let get_first_commit_from_compare = GetFirstCommitFromCompareWithGitHub {
        github_api: github_api.clone(),
    };
    let result = retrieve_four_keys::perform(
        fetch_deployments_with_github_deployment,
        fetch_deployments_with_heroku_release,
        get_first_commit_from_compare,
        project_config,
        RetrieveFourKeysExecutionContext {
            project_name: project_name.to_string(),
            since,
            until,
            environment: environment.to_string(),
        },
    )
    .await?;

    Ok(result)
}
