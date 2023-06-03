use std::{
    pin::Pin,
    task::{Context, Poll},
};

use super::{
    github_deployment_graphql::{
        DeploymentsDeploymentsNodeGraphQLResponse, DeploymentsDeploymentsStatusNodeGraphQLResponse,
    },
    github_deployment_types::{
        DeploymentNodeGraphQLResponseOrRepositoryInfo, FetchResult, GitHubDeploymentsFetcher,
    },
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
    shared::non_empty_vec::NonEmptyVec,
};
use anyhow::anyhow;
use async_trait::async_trait;
use futures::{executor::block_on, pin_mut, Stream, StreamExt};
use octocrab::Octocrab;
use pin_project::pin_project;

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
    params: DeploymentsFetcherParams,
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
        let deployment_nodes = results.data.repository_owner.repository.deployments.nodes.into_iter().filter(|node| {
            is_included_timeframe(node, &self.params.timeframe)
        }).map(DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse).collect::<Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo>>();
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

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
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

fn is_included_timeframe(
    node: &DeploymentsDeploymentsNodeGraphQLResponse,
    timeframe: &DateTimeRange,
) -> bool {
    let status = find_status(node);
    let deployed_at = status.map_or(node.created_at, |x| x.created_at);
    timeframe.is_include(&deployed_at)
}

fn has_success_status(deployment: &DeploymentNodeGraphQLResponseOrRepositoryInfo) -> bool {
    let deployment = match deployment {
        DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse(deployment) => deployment,
        _ => return true,
    };
    let statuses = deployment
        .statuses
        .nodes
        .iter()
        .map(|x| x.state.to_uppercase())
        .collect::<Vec<String>>();
    let has_success = !statuses.is_empty() && statuses.iter().any(|state| state == "SUCCESS");
    has_success
}

fn find_status(
    deployment: &DeploymentsDeploymentsNodeGraphQLResponse,
) -> Option<DeploymentsDeploymentsStatusNodeGraphQLResponse> {
    let status = deployment
        .statuses
        .nodes
        .iter()
        .find(|&x| x.state.to_uppercase() == "SUCCESS")
        .cloned();

    status
}

fn convert_to_items(
    deployment_nodes: NonEmptyVec<DeploymentNodeGraphQLResponseOrRepositoryInfo>,
) -> Vec<DeploymentLog> {
    let mut sorted: NonEmptyVec<DeploymentNodeGraphQLResponseOrRepositoryInfo> = deployment_nodes;
    sorted.sort_by_key(|a| match a {
        DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse(deployment) => deployment.created_at,
        DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryCreatedAt(created_at) => *created_at,
    });
    let (first_item, rest) = sorted.get();

    // TODO: 無理やりすぎる
    let rest = rest.iter().flat_map(|x| match x {
        DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse(deployment) => Some(deployment.clone()),
        DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryCreatedAt(_) => None,
    }).collect::<Vec<DeploymentsDeploymentsNodeGraphQLResponse>>();

    let first_commit: BaseCommitShaOrRepositoryInfo = match first_item {
        DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse(item) => BaseCommitShaOrRepositoryInfo::BaseCommitSha(item.commit.sha),
        DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryCreatedAt(created_at) => BaseCommitShaOrRepositoryInfo::RepositoryCreatedAt(created_at),
    };

    let deployment_items = rest
        .iter()
        .scan(
            first_commit,
            |previous: &mut BaseCommitShaOrRepositoryInfo,
             deployment: &DeploymentsDeploymentsNodeGraphQLResponse| {
                let status = find_status(deployment);
                let commit_item = Commit {
                    sha: deployment.clone().commit.sha,
                    message: deployment.clone().commit.message,
                    resource_path: deployment.clone().commit.commit_resource_path,
                    committed_at: deployment.clone().commit.committed_date,
                    creator_login: deployment.clone().creator.login,
                };
                let deployment_item = DeploymentLog {
                    info: DeploymentInfo::GithubDeployment {
                        id: deployment.clone().id,
                    },
                    head_commit: commit_item.clone(),
                    base: previous.clone(),
                    creator_login: deployment.clone().creator.login,
                    deployed_at: status.map_or(deployment.created_at, |x| x.created_at),
                };
                *previous = BaseCommitShaOrRepositoryInfo::BaseCommitSha(commit_item.sha);
                Some(deployment_item)
            },
        )
        .collect::<Vec<DeploymentLog>>();

    deployment_items
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
            params,
        };
        let stream = GitHubDeploymentsFetcherStream {
            fetcher,
            after: None,
            has_next_page: true,
        };
        pin_mut!(stream);
        let mut deployment_nodes: Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo> = Vec::new();
        while let Some(response) = stream.next().await {
            let data: Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo> = response
                .data
                .into_iter()
                .filter(has_success_status)
                .collect();
            deployment_nodes = [&deployment_nodes[..], &data[..]].concat();

            if !response.has_next_page {
                let repo_created_at =
                    get_created_at(&self.github_personal_token, &self.github_owner_repo)
                        .await
                        .map_err(|e| anyhow!(e))
                        .map_err(DeploymentsFetcherError::GetRepositoryCreatedAtError)?;
                log::debug!("repo_created_at: {:#?}", repo_created_at);
                deployment_nodes.push(
                    DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryCreatedAt(
                        repo_created_at,
                    ),
                );
            }
        }

        // .into_iter()
        // .filter(has_success_status)
        // .collect::<Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo>>();
        let non_empty_nodes = NonEmptyVec::new(deployment_nodes)
            .map_err(|e| anyhow::anyhow!(e))
            .map_err(DeploymentsFetcherError::DeploymentsFetcherResultIsEmptyList)?;
        let deployment_items = convert_to_items(non_empty_nodes);

        Ok(deployment_items)
    }
}
