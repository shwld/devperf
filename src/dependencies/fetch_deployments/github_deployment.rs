use super::interface::{
    CommitItem, CommitOrRepositoryInfo, DeploymentItem, FetchDeployments, FetchDeploymentsError,
    FetchDeploymentsParams, RepositoryInfo,
};
use crate::{dependencies::github_api::GitHubAPI, shared::non_empty_vec::NonEmptyVec};
use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

fn deployments_query(owner: &str, repo: &str, environment: &str, after: Option<String>) -> String {
    let query = format!("
        query {{
            repository_owner: repositoryOwner(login: \"{owner}\") {{
                repository(name: \"{repo}\") {{
                    deployments(first: 100, environments: [\"{environment}\"], orderBy: {{field: CREATED_AT, direction: DESC}}{after}) {{
                        nodes {{
                            id
                            commit {{
                                id
                                sha: oid
                                message
                                commit_resource_path: commitResourcePath
                                committed_date: committedDate
                            }}
                            task
                            environment
                            original_environment: originalEnvironment
                            description
                            created_at: createdAt
                            updated_at: updatedAt
                            creator {{
                                login
                            }}
                            statuses(first: 20) {{
                                nodes {{
                                    id
                                    state
                                    description
                                    environment_url: environmentUrl
                                    log_url: logUrl
                                    created_at: createdAt
                                    updated_at: updatedAt
                                    creator {{
                                        login
                                    }}
                                }}
                                page_info: pageInfo {{
                                    end_cursor: endCursor
                                    has_next_page: hasNextPage
                                }}
                            }}
                        }}
                        page_info: pageInfo {{
                            end_cursor: endCursor
                            has_next_page: hasNextPage
                        }}
                    }}
                }}
            }}
        }}
    ", owner = owner, repo = repo, after = after.map_or_else(|| "".to_owned(), |cursor| format!(", after: \"{}\"", cursor)));

    query
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsGraphQLResponse {
    pub data: DeploymentsDataGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsDataGraphQLResponse {
    pub repository_owner: DeploymentsRepositoryOwnerGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsRepositoryOwnerGraphQLResponse {
    pub repository: DeploymentsRepositoryGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsRepositoryGraphQLResponse {
    pub deployments: DeploymentsDeploymentsGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsDeploymentsGraphQLResponse {
    pub nodes: Vec<DeploymentsDeploymentsNodeGraphQLResponse>,
    pub page_info: DeploymentsPageInfoGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsDeploymentsNodeGraphQLResponse {
    pub id: String,
    pub commit: DeploymentsCommitGraphQLResponse,
    pub task: String,
    pub environment: String,
    pub description: Option<String>,
    pub original_environment: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub creator: DeploymentsCreatorGraphQLResponse,
    pub statuses: DeploymentsDeploymentsStatusesGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsCommitGraphQLResponse {
    pub id: String,
    pub sha: String,
    pub message: String,
    pub commit_resource_path: String,
    pub committed_date: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsPageInfoGraphQLResponse {
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsDeploymentsStatusesGraphQLResponse {
    pub nodes: Vec<DeploymentsDeploymentsStatusNodeGraphQLResponse>,
    pub page_info: DeploymentsPageInfoGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsDeploymentsStatusNodeGraphQLResponse {
    pub id: String,
    pub state: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub environment_url: Option<String>,
    pub log_url: Option<String>,
    pub creator: DeploymentsCreatorGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsCreatorGraphQLResponse {
    pub login: String,
}

async fn get_created_at(
    github_api: GitHubAPI,
    owner: &str,
    repo: &str,
) -> Result<chrono::DateTime<chrono::Utc>, FetchDeploymentsError> {
    let github_api_client = github_api
        .clone()
        .get_client()
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(FetchDeploymentsError::CreateAPIClientError)?;
    let result = github_api_client
        .repos(owner, repo)
        .get()
        .await
        .map(|r| r.created_at)
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(FetchDeploymentsError::GetRepositoryCreatedAtError)?;
    let created_at = result.ok_or(FetchDeploymentsError::RepositoryNotFound(format!(
        "{}/{}",
        owner.to_owned(),
        repo.to_owned()
    )))?;

    Ok(created_at)
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)] // most are HerokuRelease
pub enum DeploymentNodeGraphQLResponseOrRepositoryInfo {
    DeploymentsDeploymentsNodeGraphQLResponse(DeploymentsDeploymentsNodeGraphQLResponse),
    RepositoryInfo(RepositoryInfo),
}

async fn fetch_deployments(
    github_api: GitHubAPI,
    params: FetchDeploymentsParams,
) -> Result<Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo>, FetchDeploymentsError> {
    let github_api_client = github_api
        .clone()
        .get_client()
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(FetchDeploymentsError::CreateAPIClientError)?;
    let mut after: Option<String> = None;
    let mut has_next_page = true;
    let mut deployment_nodes: Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo> = Vec::new();

    // 全ページ取得
    while has_next_page {
        let query = deployments_query(&params.owner, &params.repo, &params.environment, after);
        let results: DeploymentsGraphQLResponse = github_api_client
            .graphql(&query)
            .await
            .map_err(|e| anyhow!(e))
            .map_err(FetchDeploymentsError::FetchError)?;
        let new_nodes = results.data.repository_owner.repository.deployments.nodes.into_iter().map(DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse).collect::<Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo>>();
        deployment_nodes = [&deployment_nodes[..], &new_nodes[..]].concat();
        has_next_page = results
            .data
            .repository_owner
            .repository
            .deployments
            .page_info
            .has_next_page;
        after = results
            .data
            .repository_owner
            .repository
            .deployments
            .page_info
            .end_cursor;

        log::debug!("has_next_page: {:#?}", has_next_page);
        // 初回デプロイとリードタイムを比較するためのリポジトリ作成日を取得
        if !has_next_page {
            let repo_creatd_at = get_created_at(github_api.clone(), &params.owner, &params.repo)
                .await
                .map_err(|e| anyhow!(e))
                .map_err(FetchDeploymentsError::GetRepositoryCreatedAtError)?;
            log::debug!("repo_creatd_at: {:#?}", repo_creatd_at);
            deployment_nodes.push(
                DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryInfo(RepositoryInfo {
                    created_at: repo_creatd_at,
                }),
            );
        }
    }
    log::debug!("deployment_nodes: {:#?}", deployment_nodes);

    Ok(deployment_nodes)
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
) -> Vec<DeploymentItem> {
    let mut sorted: NonEmptyVec<DeploymentNodeGraphQLResponseOrRepositoryInfo> = deployment_nodes;
    sorted.sort_by_key(|a| match a {
        DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse(deployment) => deployment.created_at,
        DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryInfo(info) => info.created_at,
    });
    let (first_item, rest) = sorted.get();

    // TODO: 無理やりすぎる
    let rest = rest.iter().flat_map(|x| match x {
        DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse(deployment) => Some(deployment.clone()),
        DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryInfo(_info) => None,
    }).collect::<Vec<DeploymentsDeploymentsNodeGraphQLResponse>>();

    let first_commit: CommitOrRepositoryInfo = match first_item {
        DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse(item) => CommitOrRepositoryInfo::Commit(CommitItem {
            sha: item.id.clone(),
            message: item.commit.message.clone(),
            resource_path: item.commit.commit_resource_path.clone(),
            committed_at: item.commit.committed_date,
            creator_login: item.creator.login,
        }),
        DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryInfo(info) => CommitOrRepositoryInfo::RepositoryInfo(info),
    };

    let deployment_items = rest
        .iter()
        .scan(
            first_commit,
            |previous: &mut CommitOrRepositoryInfo,
             deployment: &DeploymentsDeploymentsNodeGraphQLResponse| {
                let status = find_status(deployment);
                let commit_item = CommitItem {
                    sha: deployment.clone().commit.sha,
                    message: deployment.clone().commit.message,
                    resource_path: deployment.clone().commit.commit_resource_path,
                    committed_at: deployment.clone().commit.committed_date,
                    creator_login: deployment.clone().creator.login,
                };
                let deployment_item = DeploymentItem {
                    id: deployment.clone().id,
                    head_commit: commit_item.clone(),
                    base: previous.clone(),
                    creator_login: deployment.clone().creator.login,
                    deployed_at: status.map_or(deployment.created_at, |x| x.created_at),
                };
                *previous = CommitOrRepositoryInfo::Commit(commit_item);
                Some(deployment_item)
            },
        )
        .collect::<Vec<DeploymentItem>>();

    deployment_items
}

pub struct FetchDeploymentsWithGithubDeployment {
    pub github_api: GitHubAPI,
}
#[async_trait]
impl FetchDeployments for FetchDeploymentsWithGithubDeployment {
    async fn perform(
        &self,
        params: FetchDeploymentsParams,
    ) -> Result<Vec<DeploymentItem>, FetchDeploymentsError> {
        let deployment_nodes = fetch_deployments(self.github_api.clone(), params)
            .await?
            .into_iter()
            .filter(has_success_status)
            .collect::<Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo>>();
        log::debug!("deployment_nodes: {:#?}", deployment_nodes);
        let non_empty_nodes = NonEmptyVec::new(deployment_nodes)
            .map_err(|e| anyhow::anyhow!(e))
            .map_err(FetchDeploymentsError::FetchDeploymentsResultIsEmptyList)?;
        log::debug!("non_empty_nodes: {:#?}", non_empty_nodes);
        let deployment_items = convert_to_items(non_empty_nodes);
        log::debug!("deployment_items: {:#?}", deployment_items);

        Ok(deployment_items)
    }
}
