use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::{
    common_types::date_time_range::DateTimeRange,
    dependencies::{
        deployments_fetcher::{
            github_deployment::DeploymentsFetcherWithGithubDeployment,
            github_merged_pull::DeploymentsFetcherWithGithubMergedPullRequest,
            heroku_release::DeploymentsFetcherWithHerokuRelease,
        },
        project_config_io::reader::{
            interface::ProjectConfigIOReader, settings_toml::ProjectConfigIOReaderWithSettingsToml,
        },
        two_commits_comparer::github::TwoCommitsComparerWithGitHub,
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
    let timeframe = DateTimeRange::new(since, until)?;
    let context = RetrieveFourKeysExecutionContext {
        project: RetrieveFourKeysExecutionContextDto::build_context(project_config_dto.clone())?,
        timeframe,
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
            let two_commits_comparer = TwoCommitsComparerWithGitHub {
                github_personal_token: config.github_personal_token.clone(),
                github_owner_repo: config.github_owner_repo,
            };
            let workflow = RetrieveFourKeysWorkflow {
                deployments_fetcher,
                two_commits_comparer,
            };
            workflow.retrieve_four_keys(context.clone())
        }
        ProjectCreated::GitHubDeployment(config) => {
            log::info!("GitHub deployment project detected");
            let deployments_fetcher = DeploymentsFetcherWithGithubDeployment {
                github_personal_token: config.github_personal_token.clone(),
                github_owner_repo: config.github_owner_repo.clone(),
                environment: config.github_deployment_environment.clone(),
            };
            let two_commits_comparer = TwoCommitsComparerWithGitHub {
                github_personal_token: config.github_personal_token.clone(),
                github_owner_repo: config.github_owner_repo,
            };
            let workflow = RetrieveFourKeysWorkflow {
                deployments_fetcher,
                two_commits_comparer,
            };
            workflow.retrieve_four_keys(context)
        }
        ProjectCreated::GitHubPullRequest(config) => {
            log::info!("GitHub pull request project detected");
            let deployments_fetcher = DeploymentsFetcherWithGithubMergedPullRequest {
                github_personal_token: config.github_personal_token.clone(),
                github_owner_repo: config.github_owner_repo.clone(),
                deploy_trigger_branch: config.github_deploy_branch_name,
            };
            let two_commits_comparer = TwoCommitsComparerWithGitHub {
                github_personal_token: config.github_personal_token.clone(),
                github_owner_repo: config.github_owner_repo,
            };
            let workflow = RetrieveFourKeysWorkflow {
                deployments_fetcher,
                two_commits_comparer,
            };
            workflow.retrieve_four_keys(context)
        }
    }
    .await?;

    write_standard_out_from_events(events);
    Ok(())
}
