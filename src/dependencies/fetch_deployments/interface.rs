use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;

pub struct FetchDeploymentsParams {
    pub owner: String,
    pub repo: String,
    pub environment: String,
    pub since: Option<DateTime<Utc>>,
}

#[derive(Debug, Error)]
pub enum FetchDeploymentsError {
    #[error("Create API client error")]
    CreateAPIClientError(#[source] anyhow::Error),
    #[error("Fetch deployments error")]
    FetchDeploymentsError(#[source] anyhow::Error),
    #[error("Cannot get repository")]
    RepositoryNotFound(String),
    #[error("Fetch deployments result is empty list")]
    FetchDeploymentsResultIsEmptyList(#[source] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct CommitItem {
    pub sha: String,
    pub message: String,
    pub resource_path: String,
    pub committed_at: DateTime<Utc>,
    pub creator_login: String,
}

#[derive(Debug, Clone)]
pub struct DeploymentItem {
    pub id: String,
    pub head_commit: CommitItem,
    pub base_commit: CommitItem,
    pub creator_login: String,
    pub deployed_at: DateTime<Utc>,
}

#[async_trait]
pub trait FetchDeployments {
    async fn perform(&self, params: FetchDeploymentsParams) -> Result<Vec<DeploymentItem>, FetchDeploymentsError>;
}
