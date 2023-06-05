use async_trait::async_trait;

use super::{
    github_deployment_graphql::DeploymentsDeploymentsNodeGraphQLResponse,
    interface::DeploymentsFetcherError,
};

// ---------------------------
// Fetching step
// ---------------------------
pub(super) struct FetchResult {
    pub(super) data: Vec<DeploymentsDeploymentsNodeGraphQLResponse>,
    pub(super) after: Option<String>,
    pub(super) has_next_page: bool,
}

#[async_trait]
pub(super) trait GitHubDeploymentsFetcher {
    async fn fetch(&self, after: Option<String>) -> Result<FetchResult, DeploymentsFetcherError>;
}
