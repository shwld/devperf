use chrono::{DateTime, Utc};

use super::github_deployment::DeploymentsDeploymentsNodeGraphQLResponse;

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)] // most are HerokuRelease
pub enum DeploymentNodeGraphQLResponseOrRepositoryInfo {
    DeploymentsDeploymentsNodeGraphQLResponse(DeploymentsDeploymentsNodeGraphQLResponse),
    RepositoryCreatedAt(DateTime<Utc>),
}
