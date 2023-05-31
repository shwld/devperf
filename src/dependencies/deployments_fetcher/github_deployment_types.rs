use chrono::{DateTime, Utc};

use super::github_deployment_graphql::DeploymentsDeploymentsNodeGraphQLResponse;

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)] // most are HerokuRelease
pub(super) enum DeploymentNodeGraphQLResponseOrRepositoryInfo {
    DeploymentsDeploymentsNodeGraphQLResponse(DeploymentsDeploymentsNodeGraphQLResponse),
    RepositoryCreatedAt(DateTime<Utc>),
}
