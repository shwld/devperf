use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common_types::{commit::Commit, date_time_range::DateTimeRange};

// Input
pub struct DeploymentsFetcherParams {
    pub timeframe: DateTimeRange,
}

#[derive(Debug, Clone)]
pub enum BaseCommitShaOrRepositoryInfo {
    BaseCommitSha(String),
    RepositoryCreatedAt(DateTime<Utc>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeploymentInfo {
    GithubDeployment {
        id: String,
    },
    GithubMergedPullRequest {
        id: String,
        number: u64,
        title: String,
    },
    HerokuRelease {
        id: String,
        version: u64,
    },
}

#[derive(Debug, Clone)]
pub struct DeploymentLog {
    pub info: DeploymentInfo,
    pub head_commit: Commit,
    pub base: BaseCommitShaOrRepositoryInfo,
    pub creator_login: String,
    pub deployed_at: DateTime<Utc>,
}

// Errors
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

// Workflow
#[async_trait]
pub trait DeploymentsFetcher {
    async fn fetch(
        &self,
        params: DeploymentsFetcherParams,
    ) -> Result<Vec<DeploymentLog>, DeploymentsFetcherError>;
}
