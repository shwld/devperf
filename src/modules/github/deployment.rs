use serde::{Deserialize, Serialize};
use crate::modules::types::deployment_metric::{DeploymentItem};

pub async fn list(owner: &str, repo: &str, environment: &str) -> Result<Vec<DeploymentItem>, octocrab::Error> {
    let crab = super::client::create_github_client().await;
    let mut after: Option<String> = None;
    let mut has_next_page = true;
    let mut deployment_nodes: Vec<super::deployment::DeploymentsDeploymentsNodeGraphQLResponse> = Vec::new();

    // 全ページ取得
    let mut loop_count = 0;
    while has_next_page && loop_count < 50 {
        let query = deployments_query(owner, repo, environment, after);
        let results: DeploymentsGraphQLResponse = crab.graphql(&query).await.expect("Could not get deployments");
        deployment_nodes = [&deployment_nodes[..], &results.data.repository_owner.repository.deployments.nodes[..]].concat();
        has_next_page = results.data.repository_owner.repository.deployments.page_info.has_next_page;
        after = results.data.repository_owner.repository.deployments.page_info.end_cursor;
        loop_count += 1;
        log::debug!("loop{}: {}", loop_count, deployment_nodes.len());
    }

    let mut deployments: Vec<DeploymentItem> = Vec::new();

    for deployment in deployment_nodes {
        let statuses = deployment.statuses.nodes.clone();
        let states = statuses.iter().map(|x| x.state.to_uppercase()).collect::<Vec<String>>();
        if states.len() == 0 || states.iter().all(|state| state != "SUCCESS") {
            continue;
        }
        let status = statuses.iter().find(|&x| x.state.to_uppercase() == "SUCCESS").expect("Could not get status");
        let deployment = DeploymentItem {
            id: deployment.id,
            commit_sha: deployment.commit.id,
            commit_message: deployment.commit.message,
            commit_resource_path: deployment.commit.commit_resource_path,
            committed_at: deployment.commit.committed_date,
            creator_login: deployment.creator.login,
            deployed_at: status.created_at,
        };

        log::debug!("deployment added {:?}", deployment.commit_message);
        deployments.push(deployment);
    }

    Ok(deployments)
}

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
