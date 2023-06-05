use async_trait::async_trait;
use chrono::{DateTime, Utc};
use octocrab::Octocrab;

use crate::common_types::{
    date_time_range::DateTimeRange, github_personal_token::ValidatedGitHubPersonalToken,
};

use super::{
    github_deployment_graphql::{
        DeploymentsDeploymentsNodeGraphQLResponse, DeploymentsDeploymentsStatusNodeGraphQLResponse,
    },
    interface::{BaseCommitShaOrRepositoryInfo, DeploymentLog, DeploymentsFetcherError},
};

// ---------------------------
// Fetching step
// ---------------------------
pub(super) type GetClient =
    fn(&ValidatedGitHubPersonalToken) -> Result<Octocrab, DeploymentsFetcherError>;

pub(super) struct FetchResult {
    pub(super) data: Vec<DeploymentsDeploymentsNodeGraphQLResponse>,
    pub(super) after: Option<String>,
    pub(super) has_next_page: bool,
}

#[async_trait]
pub(super) trait GitHubDeploymentsFetcher {
    async fn fetch(&self, after: Option<String>) -> Result<FetchResult, DeploymentsFetcherError>;
}

// ---------------------------
// Filtering step
// ---------------------------
pub(super) type GetDeployedAt =
    fn(deployment_node: &DeploymentsDeploymentsNodeGraphQLResponse) -> DateTime<Utc>;

pub(super) type GetSucceededStatuses = fn(
    deployment_node: &DeploymentsDeploymentsNodeGraphQLResponse,
) -> Vec<&DeploymentsDeploymentsStatusNodeGraphQLResponse>;

// ---------------------------
// Collecting step
// ---------------------------
pub(super) type SliceDeploymentNodes = fn(
    nodes: Vec<DeploymentsDeploymentsNodeGraphQLResponse>,
    timeframe: &DateTimeRange,
) -> (
    Option<DeploymentsDeploymentsNodeGraphQLResponse>,
    Vec<DeploymentsDeploymentsNodeGraphQLResponse>,
);

pub(super) type CollectToLogs = fn(
    first_item: BaseCommitShaOrRepositoryInfo,
    deployment_nodes: Vec<DeploymentsDeploymentsNodeGraphQLResponse>,
) -> Vec<DeploymentLog>;
