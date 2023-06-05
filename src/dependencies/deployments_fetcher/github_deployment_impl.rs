use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::{executor::block_on, pin_mut, Stream, StreamExt};
use octocrab::Octocrab;
use pin_project::pin_project;
use std::{
    cmp::Ordering,
    pin::Pin,
    task::{Context, Poll},
};

use super::{
    github_deployment_graphql::{
        DeploymentsDeploymentsNodeGraphQLResponse, DeploymentsDeploymentsStatusNodeGraphQLResponse,
    },
    github_deployment_types::{FetchResult, GitHubDeploymentsFetcher},
    interface::{
        BaseCommitShaOrRepositoryInfo, DeploymentInfo, DeploymentLog, DeploymentsFetcher,
        DeploymentsFetcherError, DeploymentsFetcherParams,
    },
};
use crate::{
    common_types::{
        commit::Commit, date_time_range::DateTimeRange,
        github_deployment_environment::ValidatedGitHubDeploymentEnvironment,
        github_owner_repo::ValidatedGitHubOwnerRepo,
        github_personal_token::ValidatedGitHubPersonalToken,
    },
    dependencies::deployments_fetcher::{
        github_deployment_graphql::{deployments_query, DeploymentsGraphQLResponse},
        shared::get_created_at,
    },
};

fn get_client(
    github_personal_token: &ValidatedGitHubPersonalToken,
) -> Result<Octocrab, DeploymentsFetcherError> {
    let client = Octocrab::builder()
        .personal_token(github_personal_token.to_string())
        .build()
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(DeploymentsFetcherError::CreateAPIClientError)?;

    Ok(client)
}

struct GitHubDeploymentsFetcherImpl {
    github_personal_token: ValidatedGitHubPersonalToken,
    github_owner_repo: ValidatedGitHubOwnerRepo,
    environment: ValidatedGitHubDeploymentEnvironment,
}
#[async_trait]
impl GitHubDeploymentsFetcher for GitHubDeploymentsFetcherImpl {
    async fn fetch(&self, after: Option<String>) -> Result<FetchResult, DeploymentsFetcherError> {
        let github_client = get_client(&self.github_personal_token)?;

        let query = deployments_query(&self.github_owner_repo, &self.environment, after);

        let results: DeploymentsGraphQLResponse = github_client
            .graphql(&query)
            .await
            .map_err(|e| anyhow!(e))
            .map_err(DeploymentsFetcherError::FetchError)?;
        let deployment_nodes: Vec<DeploymentsDeploymentsNodeGraphQLResponse> = results
            .data
            .repository_owner
            .repository
            .deployments
            .nodes
            .into_iter()
            .filter(|node| has_succeeded_status(succeeded_statuses(node)))
            .collect();
        let has_next_page = results
            .data
            .repository_owner
            .repository
            .deployments
            .page_info
            .has_next_page;
        let after = results
            .data
            .repository_owner
            .repository
            .deployments
            .page_info
            .end_cursor;

        log::debug!("has_next_page: {:#?}", has_next_page);

        Ok(FetchResult {
            data: deployment_nodes,
            after,
            has_next_page,
        })
    }
}

#[pin_project]
struct GitHubDeploymentsFetcherStream<T: GitHubDeploymentsFetcher> {
    fetcher: T,
    after: Option<String>,
    has_next_page: bool,
}
impl<T: GitHubDeploymentsFetcher> Stream for GitHubDeploymentsFetcherStream<T> {
    type Item = FetchResult;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if !self.has_next_page {
            return Poll::Ready(None);
        }
        if let Ok(response) = block_on(self.fetcher.fetch(self.after.clone())) {
            let this = self.project();
            *this.after = response.after.clone();
            *this.has_next_page = response.has_next_page;
            Poll::Ready(Some(response))
        } else {
            Poll::Ready(None)
        }
    }
}

fn succeeded_statuses(
    deployment_node: &DeploymentsDeploymentsNodeGraphQLResponse,
) -> Vec<&DeploymentsDeploymentsStatusNodeGraphQLResponse> {
    let statuses: Vec<&DeploymentsDeploymentsStatusNodeGraphQLResponse> = deployment_node
        .statuses
        .nodes
        .iter()
        .filter(|x| x.state.to_uppercase() == "SUCCESS")
        .collect();

    statuses
}
fn has_succeeded_status(statuses: Vec<&DeploymentsDeploymentsStatusNodeGraphQLResponse>) -> bool {
    !statuses.is_empty()
}

fn get_deployed_at(deployment_node: &DeploymentsDeploymentsNodeGraphQLResponse) -> DateTime<Utc> {
    let binding = succeeded_statuses(deployment_node);
    let status = binding.first();
    status.map_or(deployment_node.created_at, |x| x.created_at)
}

fn slice_deployment_nodes(
    nodes: Vec<DeploymentsDeploymentsNodeGraphQLResponse>,
    timeframe: &DateTimeRange,
) -> (
    Option<DeploymentsDeploymentsNodeGraphQLResponse>,
    Vec<DeploymentsDeploymentsNodeGraphQLResponse>,
) {
    let ranged_nodes: Vec<DeploymentsDeploymentsNodeGraphQLResponse> = nodes
        .clone()
        .into_iter()
        .filter(|node| timeframe.is_include(&get_deployed_at(node)))
        .collect();
    let last_date_before_since = nodes
        .into_iter()
        .filter(|node| {
            matches!(
                timeframe.get_since().cmp(&get_deployed_at(node)),
                Ordering::Greater
            )
        })
        .last();
    (last_date_before_since, ranged_nodes)
}

fn collect_to_logs(
    first_item: BaseCommitShaOrRepositoryInfo,
    deployment_nodes: Vec<DeploymentsDeploymentsNodeGraphQLResponse>,
) -> Vec<DeploymentLog> {
    let deployment_logs = deployment_nodes
        .iter()
        .scan(
            first_item,
            |previous: &mut BaseCommitShaOrRepositoryInfo,
             deployment: &DeploymentsDeploymentsNodeGraphQLResponse| {
                let commit = Commit {
                    sha: deployment.clone().commit.sha,
                    message: deployment.clone().commit.message,
                    resource_path: deployment.clone().commit.commit_resource_path,
                    committed_at: deployment.clone().commit.committed_date,
                    creator_login: deployment.clone().creator.login,
                };
                let deployment_log = DeploymentLog {
                    info: DeploymentInfo::GithubDeployment {
                        id: deployment.clone().id,
                    },
                    head_commit: commit.clone(),
                    base: previous.clone(),
                    creator_login: deployment.clone().creator.login,
                    deployed_at: get_deployed_at(deployment),
                };
                *previous = BaseCommitShaOrRepositoryInfo::BaseCommitSha(commit.sha);
                Some(deployment_log)
            },
        )
        .collect::<Vec<DeploymentLog>>();

    deployment_logs
}

pub struct DeploymentsFetcherWithGithubDeployment {
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub environment: ValidatedGitHubDeploymentEnvironment,
}
#[async_trait]
impl DeploymentsFetcher for DeploymentsFetcherWithGithubDeployment {
    async fn fetch(
        &self,
        params: DeploymentsFetcherParams,
    ) -> Result<Vec<DeploymentLog>, DeploymentsFetcherError> {
        let fetcher = GitHubDeploymentsFetcherImpl {
            github_personal_token: self.github_personal_token.to_owned(),
            github_owner_repo: self.github_owner_repo.to_owned(),
            environment: self.environment.to_owned(),
        };
        let stream = GitHubDeploymentsFetcherStream {
            fetcher,
            after: None,
            has_next_page: true,
        };
        pin_mut!(stream);
        let mut deployment_nodes: Vec<DeploymentsDeploymentsNodeGraphQLResponse> = Vec::new();
        let mut last_commit_before_since_or_repository_created_at: Option<
            BaseCommitShaOrRepositoryInfo,
        > = None;
        // HACK: more better logic
        while let Some(response) = stream.next().await {
            let (last_node_before_since, nodes) = slice_deployment_nodes(
                [&deployment_nodes[..], &response.data[..]].concat(),
                &params.timeframe,
            );
            deployment_nodes = nodes;

            if last_node_before_since.is_some() {
                break;
            }

            if !response.has_next_page {
                let repo_created_at =
                    get_created_at(&self.github_personal_token, &self.github_owner_repo)
                        .await
                        .map_err(|e| anyhow!(e))
                        .map_err(DeploymentsFetcherError::GetRepositoryCreatedAtError)?;
                log::debug!("repo_created_at: {:#?}", repo_created_at);
                last_commit_before_since_or_repository_created_at = Some(
                    BaseCommitShaOrRepositoryInfo::RepositoryCreatedAt(repo_created_at),
                );
            }
        }

        if let Some(last_commit_before_since_or_repository_created_at) =
            last_commit_before_since_or_repository_created_at
        {
            let deployment_logs = collect_to_logs(
                last_commit_before_since_or_repository_created_at,
                deployment_nodes,
            );

            Ok(deployment_logs)
        } else {
            Err(
                DeploymentsFetcherError::DeploymentsFetcherResultIsEmptyList(anyhow!(
                    "Deployments fetcher result is empty list"
                )),
            )
        }
    }
}
