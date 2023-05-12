use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;

pub struct DeploymentsFetcherParams {
    pub since: Option<DateTime<Utc>>,
}

#[derive(Debug, Error)]
pub enum DeploymentsFetcherError {
    #[error("Create API client error")]
    CreateAPIClientError(#[source] anyhow::Error),
    #[error("Fetch deployments error")]
    FetchError(#[source] anyhow::Error),
    #[error("Get commit error")]
    CommitIsNotFound(#[source] anyhow::Error),
    #[error("Cannot get repository created at")]
    GetRepositoryCreatedAtError(#[source] anyhow::Error),
    #[error("Fetch deployments result is empty list")]
    DeploymentsFetcherResultIsEmptyList(#[source] anyhow::Error),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
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
pub struct RepositoryInfo {
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum CommitOrRepositoryInfo {
    Commit(CommitItem),
    RepositoryInfo(RepositoryInfo),
}

#[derive(Debug, Clone)]
pub struct DeploymentItem {
    pub id: String,
    pub head_commit: CommitItem,
    pub base: CommitOrRepositoryInfo,
    pub creator_login: String,
    pub deployed_at: DateTime<Utc>,
}

#[async_trait]
pub trait DeploymentsFetcher {
    async fn fetch(
        &self,
        params: DeploymentsFetcherParams,
    ) -> Result<Vec<DeploymentItem>, DeploymentsFetcherError>;
}
