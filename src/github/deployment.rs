use std::ops::Index;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::common_types::DeploymentItem;
use super::client::{GitHubClientError, GetGitHubPersonalToken};

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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub creator: DeploymentsCreatorGraphQLResponse,
    pub statuses: DeploymentsDeploymentsStatusesGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsCommitGraphQLResponse {
    pub id: String,
    pub message: String,
    pub commit_resource_path: String,
    pub committed_date: chrono::DateTime<chrono::Utc>,
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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub environment_url: Option<String>,
    pub log_url: Option<String>,
    pub creator: DeploymentsCreatorGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeploymentsCreatorGraphQLResponse {
    pub login: String,
}

pub struct GitHubDeploymentListFetchOptions {
    pub owner: String,
    pub repo: String,
    pub environment: String,
    pub since: Option<DateTime<Utc>>,
}

async fn fetch_deployments(get_github_personal_token: GetGitHubPersonalToken, options: GitHubDeploymentListFetchOptions) -> Result<Vec<DeploymentsDeploymentsNodeGraphQLResponse>, GitHubClientError> {
  let client = super::client::create_github_client(get_github_personal_token)?;
    let mut after: Option<String> = None;
    let mut has_next_page = true;
    let mut deployment_nodes: Vec<DeploymentsDeploymentsNodeGraphQLResponse> = Vec::new();

    // 全ページ取得
    let mut loop_count = 0;
    while has_next_page && loop_count < 50 {
        let query = deployments_query(&options.owner, &options.repo, &options.environment, after);
        let results: DeploymentsGraphQLResponse = client.graphql(&query).await.expect("Could not get deployments");
        deployment_nodes = [&deployment_nodes[..], &results.data.repository_owner.repository.deployments.nodes[..]].concat();
        has_next_page = results.data.repository_owner.repository.deployments.page_info.has_next_page;
        after = results.data.repository_owner.repository.deployments.page_info.end_cursor;
        loop_count += 1;
        log::debug!("loop{}: {}", loop_count, deployment_nodes.len());
        if let Some(since) = options.since {
            if let Some(index) = deployment_nodes.iter().position(|x| x.created_at < since) {
                // sinceより古いデータが1つ以上ある状態なら、ループを抜ける
                if deployment_nodes.len() > index {
                    break;
                }
            }
        }

    }

    Ok(deployment_nodes)
}

fn has_success_status(deployment: &DeploymentsDeploymentsNodeGraphQLResponse) -> bool {
    let statuses = deployment.statuses.nodes.iter().map(|x| x.state.to_uppercase()).collect::<Vec<String>>();
    let has_success = statuses.len() > 0 && statuses.iter().any(|state| state == "SUCCESS");
    has_success
}

fn to_item(deployment: DeploymentsDeploymentsNodeGraphQLResponse) -> DeploymentItem {
    let status = deployment.statuses.nodes.iter().find(|&x| x.state.to_uppercase() == "SUCCESS");
    let deployment = DeploymentItem {
        id: deployment.id,
        head_commit_sha: deployment.commit.id,
        head_commit_message: deployment.commit.message,
        head_commit_resource_path: deployment.commit.commit_resource_path,
        head_committed_at: deployment.commit.committed_date,
        creator_login: deployment.creator.login,
        deployed_at: status.map_or(deployment.created_at, |x| x.created_at),
    };
    deployment
}

pub async fn list(get_github_personal_token: GetGitHubPersonalToken, options: GitHubDeploymentListFetchOptions) -> Result<Vec<DeploymentItem>, GitHubClientError> {
    let deployments = fetch_deployments(get_github_personal_token, options)
        .await?
        .iter()
        .filter(|&x| has_success_status(x))
        .map(|x| to_item(x.clone()))
        .collect();

    Ok(deployments)
}
