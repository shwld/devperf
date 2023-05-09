use async_trait::async_trait;
use anyhow::{anyhow, Context as _};
use chrono::{DateTime, Utc, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use crate::{dependencies::github_api::{GitHubAPI}, common_types::NonEmptyVec};
use super::{interface::{FetchDeploymentsError, FetchDeployments, FetchDeploymentsParams, DeploymentItem, CommitItem}, get_initial_commit_item::get_initial_commit_item};

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


async fn fetch_deployments(github_api: GitHubAPI, params: FetchDeploymentsParams) -> Result<Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo>, FetchDeploymentsError> {
    let github_api_client = github_api.clone().get_client().map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::CreateAPIClientError)?;
    let mut after: Option<String> = None;
    let mut has_next_page = true;
    let mut deployment_nodes: Vec<DeploymentsDeploymentsNodeGraphQLResponse> = Vec::new();

    // 全ページ取得
    while has_next_page {
        let query = deployments_query(&params.owner, &params.repo, &params.environment, after);
        let results: DeploymentsGraphQLResponse = github_api_client.graphql(&query).await.map_err(|e| anyhow!(e)).map_err(FetchDeploymentsError::FetchDeploymentsError)?;
        deployment_nodes = [&deployment_nodes[..], &results.data.repository_owner.repository.deployments.nodes[..]].concat();
        has_next_page = results.data.repository_owner.repository.deployments.page_info.has_next_page;
        after = results.data.repository_owner.repository.deployments.page_info.end_cursor;

        // 初回デプロイと比較するための初回コミット用のデータを追加する。もうちょっとスマートに書きたい
        if !has_next_page {
            let initial_commit = get_initial_commit_item(github_api_client, &params.owner, &params.repo).await.map_err(|e| anyhow!(e)).map_err(FetchDeploymentsError::GetInitialCommitError)?;
            let time = NaiveTime::from_hms_opt(0, 0, 0).expect("Could not parse time");
            let oldest_time = NaiveDate::from_ymd_opt(1970, 1, 1).expect("invalid date").and_time(time).and_local_timezone(Utc).unwrap();
            deployment_nodes.push(DeploymentsDeploymentsNodeGraphQLResponse {
                id: "".to_owned(),
                commit: DeploymentsCommitGraphQLResponse {
                    id: initial_commit.sha,
                    message: initial_commit.message,
                    commit_resource_path: initial_commit.resource_path,
                    committed_date: initial_commit.committed_at,
                },
                task: "".to_owned(),
                environment: params.environment.clone(),
                description: None,
                original_environment: "".to_owned(),
                created_at: oldest_time.clone(),
                updated_at: oldest_time.clone(),
                creator: DeploymentsCreatorGraphQLResponse {
                    login: "".to_owned(),
                },
                statuses: DeploymentsDeploymentsStatusesGraphQLResponse {
                    nodes: Vec::new(),
                    page_info: DeploymentsPageInfoGraphQLResponse {
                        end_cursor: None,
                        has_next_page: false,
                    },
                },
            })
        }
    }

    Ok(deployment_nodes)
}

fn has_success_status(deployment: &DeploymentsDeploymentsNodeGraphQLResponse) -> bool {
    let statuses = deployment.statuses.nodes.iter().map(|x| x.state.to_uppercase()).collect::<Vec<String>>();
    let has_success = statuses.len() > 0 && statuses.iter().any(|state| state == "SUCCESS");
    has_success
}

fn find_status(deployment: &DeploymentsDeploymentsNodeGraphQLResponse) -> Option<DeploymentsDeploymentsStatusNodeGraphQLResponse> {
    let status = deployment
        .statuses
        .nodes
        .iter()
        .find(|&x| x.state.to_uppercase() == "SUCCESS")
        .map(|x| x.clone());

    status
}

fn convert_to_items(deployment_nodes: NonEmptyVec<DeploymentsDeploymentsNodeGraphQLResponse>) -> Vec<DeploymentItem> {
    let mut sorted: NonEmptyVec<DeploymentsDeploymentsNodeGraphQLResponse> = deployment_nodes.clone();
    sorted.sort_by_key(|a| a.created_at);
    let (first_item, rest) = sorted.get();
    let first_commit = CommitItem {
        sha: first_item.commit.id.clone(),
        message: first_item.commit.message.clone(),
        resource_path: first_item.commit.commit_resource_path.clone(),
        committed_at: first_item.commit.committed_date,
        creator_login: first_item.creator.login.clone(),
    };

    let deployment_items = rest
        .iter()
        .scan(first_commit, |previous: &mut CommitItem, deployment: &DeploymentsDeploymentsNodeGraphQLResponse| {
            let status = find_status(deployment);
            let commit_item = CommitItem {
                sha: deployment.clone().commit.id,
                message: deployment.clone().commit.message,
                resource_path: deployment.clone().commit.commit_resource_path,
                committed_at: deployment.clone().commit.committed_date,
                creator_login: deployment.clone().creator.login,
            };
            let deployment_item = DeploymentItem {
                id: deployment.clone().id,
                head_commit: commit_item.clone(),
                base_commit: previous.clone(),
                creator_login: deployment.clone().creator.login,
                deployed_at: status.map_or(deployment.created_at, |x| x.created_at),
            };
            *previous = commit_item;
            Some(deployment_item)
        }).collect::<Vec<DeploymentItem>>();

    deployment_items
}

pub struct FetchDeploymentsWithGithubDeployment {
    pub github_api: GitHubAPI,
}
#[async_trait]
impl FetchDeployments for FetchDeploymentsWithGithubDeployment {
    async fn perform(&self, params: FetchDeploymentsParams) -> Result<Vec<DeploymentItem>, FetchDeploymentsError> {
        let deployment_nodes = fetch_deployments(self.github_api.clone(), params)
            .await?
            .into_iter()
            .filter(|x| has_success_status(x))
            .collect::<Vec<DeploymentsDeploymentsNodeGraphQLResponse>>();
        let non_empty_nodes = NonEmptyVec::new(deployment_nodes)
            .map_err(|e| anyhow::anyhow!(e))
            .map_err(FetchDeploymentsError::FetchDeploymentsResultIsEmptyList)?;
        let deployment_items = convert_to_items(non_empty_nodes);

        Ok(deployment_items)
    }
}
