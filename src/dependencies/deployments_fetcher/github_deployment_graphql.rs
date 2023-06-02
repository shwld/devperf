use crate::common_types::{
    github_deployment_environment::ValidatedGitHubDeploymentEnvironment,
    github_owner_repo::ValidatedGitHubOwnerRepo,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub(super) fn deployments_query(
    owner_repo: &ValidatedGitHubOwnerRepo,
    environment: &ValidatedGitHubDeploymentEnvironment,
    after: Option<String>,
) -> String {
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
    ", owner = owner_repo.get_owner(), repo = owner_repo.get_repo(), environment = environment, after = after.map_or_else(|| "".to_owned(), |cursor| format!(", after: \"{}\"", cursor)));

    query
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct DeploymentsGraphQLResponse {
    pub(super) data: DeploymentsDataGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct DeploymentsDataGraphQLResponse {
    pub(super) repository_owner: DeploymentsRepositoryOwnerGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct DeploymentsRepositoryOwnerGraphQLResponse {
    pub(super) repository: DeploymentsRepositoryGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct DeploymentsRepositoryGraphQLResponse {
    pub(super) deployments: DeploymentsDeploymentsGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct DeploymentsDeploymentsGraphQLResponse {
    pub(super) nodes: Vec<DeploymentsDeploymentsNodeGraphQLResponse>,
    pub(super) page_info: DeploymentsPageInfoGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct DeploymentsDeploymentsNodeGraphQLResponse {
    pub(super) id: String,
    pub(super) commit: DeploymentsCommitGraphQLResponse,
    pub(super) task: String,
    pub(super) environment: String,
    pub(super) description: Option<String>,
    pub(super) original_environment: String,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
    pub(super) creator: DeploymentsCreatorGraphQLResponse,
    pub(super) statuses: DeploymentsDeploymentsStatusesGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct DeploymentsCommitGraphQLResponse {
    pub(super) id: String,
    pub(super) sha: String,
    pub(super) message: String,
    pub(super) commit_resource_path: String,
    pub(super) committed_date: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct DeploymentsPageInfoGraphQLResponse {
    pub(super) end_cursor: Option<String>,
    pub(super) has_next_page: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct DeploymentsDeploymentsStatusesGraphQLResponse {
    pub(super) nodes: Vec<DeploymentsDeploymentsStatusNodeGraphQLResponse>,
    pub(super) page_info: DeploymentsPageInfoGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct DeploymentsDeploymentsStatusNodeGraphQLResponse {
    pub(super) id: String,
    pub(super) state: String,
    pub(super) description: Option<String>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
    pub(super) environment_url: Option<String>,
    pub(super) log_url: Option<String>,
    pub(super) creator: DeploymentsCreatorGraphQLResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct DeploymentsCreatorGraphQLResponse {
    pub(super) login: String,
}
