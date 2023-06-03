use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::common_types::{
    github_deployment_environment::ValidatedGitHubDeploymentEnvironment,
    github_owner_repo::ValidatedGitHubOwnerRepo,
    github_personal_token::ValidatedGitHubPersonalToken,
};

use super::{
    github_deployment_graphql::DeploymentsDeploymentsNodeGraphQLResponse,
    interface::{DeploymentsFetcherError, DeploymentsFetcherParams},
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
