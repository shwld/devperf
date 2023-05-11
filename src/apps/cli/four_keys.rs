use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{
    dependencies::{
        deployments_fetcher::{
            github_deployment::DeploymentsFetcherWithGithubDeployment,
            heroku_release::DeploymentsFetcherWithHerokuRelease,
        },
        first_commit_getter::github::FirstCommitGetterWithGitHub,
        github_api::GitHubAPI,
        project_config_io::reader::{
            interface::ProjectConfigIOReader, settings_toml::ProjectConfigIOReaderWithSettingsToml,
        },
    },
    metrics_retrieving::{
        dto::RetrieveFourKeysExecutionContextDto,
        retrieve_four_keys_implementation::RetrieveFourKeysWorkflow,
        retrieve_four_keys_public_types::{
            RetrieveFourKeys, RetrieveFourKeysEvent, RetrieveFourKeysExecutionContext,
        },
    },
    project_creating::create_project_public_types::ProjectCreated,
};

fn write_standard_out_from_events(events: Vec<RetrieveFourKeysEvent>) {
    for event in events {
        match event {
            RetrieveFourKeysEvent::FourKeysMetrics(metrics) => {
                println!("{:#?}", metrics);
            }
        }
    }
}

pub async fn get_four_keys(
    project_name: &str,
    since: DateTime<Utc>,
    until: DateTime<Utc>,
    environment: &str,
) -> Result<()> {
    let config_reader = ProjectConfigIOReaderWithSettingsToml {};
    let project_config_dto = config_reader.read(project_name.to_string()).await?;
    let context = RetrieveFourKeysExecutionContext {
        project: RetrieveFourKeysExecutionContextDto::build_context(project_config_dto.clone())?,
        since,
        until,
    };
    let project_config: ProjectCreated = project_config_dto.try_into()?;

    let events = match project_config {
        ProjectCreated::HerokuRelease(config) => {
            log::info!("Heroku project detected");
            let deployments_fetcher = DeploymentsFetcherWithHerokuRelease {
                heroku_app_name: config.heroku_app_name,
                heroku_auth_token: config.heroku_auth_token,
                github_owner_repo: config.github_owner_repo.clone(),
                github_api: GitHubAPI::new(config.github_personal_token)?,
            };
            let first_commit_getter = FirstCommitGetterWithGitHub {
                github_api: GitHubAPI::new(config.github_personal_token)?,
                github_owner_repo: config.github_owner_repo,
            };
            let workflow = RetrieveFourKeysWorkflow {
                deployments_fetcher,
                first_commit_getter,
            };
            workflow.retrieve_four_keys(context.clone())
        }
        ProjectCreated::GitHubDeployment(config) => {
            log::info!("GitHub project detected");
            let deployments_fetcher = DeploymentsFetcherWithGithubDeployment {
                environment: environment.to_string(),
                github_owner_repo: config.github_owner_repo.clone(),
                github_api: GitHubAPI::new(config.github_personal_token)?,
            };
            let first_commit_getter = FirstCommitGetterWithGitHub {
                github_api: GitHubAPI::new(config.github_personal_token)?,
                github_owner_repo: config.github_owner_repo,
            };
            let workflow = RetrieveFourKeysWorkflow {
                deployments_fetcher,
                first_commit_getter,
            };
            workflow.retrieve_four_keys(context)
        }
    }
    .await?;

    write_standard_out_from_events(events);

    Ok(())
}
