use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::{
    github_deployment_graphql::DeploymentsDeploymentsNodeGraphQLResponse,
    interface::DeploymentsFetcherError,
};

// ---------------------------
// Fetching step
// ---------------------------

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)] // most are HerokuRelease
pub(super) enum DeploymentNodeGraphQLResponseOrRepositoryInfo {
    DeploymentsDeploymentsNodeGraphQLResponse(DeploymentsDeploymentsNodeGraphQLResponse),
    RepositoryCreatedAt(DateTime<Utc>),
}

pub(super) struct FetchResult {
    pub(super) data: Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo>,
    pub(super) after: Option<String>,
    pub(super) has_next_page: bool,
}

#[async_trait]
pub(super) trait GitHubDeploymentsFetcher {
    async fn fetch(&self, after: Option<String>) -> Result<FetchResult, DeploymentsFetcherError>;
}
