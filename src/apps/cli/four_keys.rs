use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{
    dependencies::{
        deployments_fetcher::{
            github_deployment::DeploymentsFetcherWithGithubDeployment,
            heroku_release::DeploymentsFetcherWithHerokuRelease,
        },
        first_commit_getter::github::FirstCommitGetterWithGitHub,
        project_config_io::reader::{
            interface::ProjectConfigIOReader, settings_toml::ProjectConfigIOReaderWithSettingsToml,
        },
    },
    metrics_retrieving::{
        dto::RetrieveFourKeysExecutionContextDto,
        retrieve_four_keys::{
            RetrieveFourKeys, RetrieveFourKeysEvent, RetrieveFourKeysExecutionContext,
            RetrieveFourKeysWorkflow,
        },
    },
    project_creating::create_project::ProjectCreated,
};

fn write_standard_out_from_events(events: Vec<RetrieveFourKeysEvent>) {
    for event in events {
        match event {
            RetrieveFourKeysEvent::RetrieveFourKeys(metrics) => {
                println!("{}", serde_json::to_string_pretty(&metrics).unwrap());
            }
        }
    }
}

pub async fn get_four_keys(
    project_name: &str,
    since: DateTime<Utc>,
    until: DateTime<Utc>,
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
                heroku_app_name: config.heroku_app_name.clone(),
                heroku_auth_token: config.heroku_auth_token.clone(),
                github_owner_repo: config.github_owner_repo.clone(),
                github_personal_token: config.github_personal_token.clone(),
            };
            let first_commit_getter = FirstCommitGetterWithGitHub {
                github_personal_token: config.github_personal_token.clone(),
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
                github_personal_token: config.github_personal_token.clone(),
                github_owner_repo: config.github_owner_repo.clone(),
                environment: config.github_deployment_environment.clone(),
            };
            let first_commit_getter = FirstCommitGetterWithGitHub {
                github_personal_token: config.github_personal_token,
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
