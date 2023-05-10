use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug)]
pub struct FirstCommitItem {
    pub sha: String,
    pub message: String,
    pub resource_path: String,
    pub committed_at: DateTime<Utc>,
    pub creator_login: String,
}

#[derive(Debug)]
pub struct FirstCommitFromCompareParams {
    pub owner: String,
    pub repo: String,
    pub base: String,
    pub head: String,
}

#[derive(Debug, Error)]
pub enum GetFirstCommitFromCompareError {
    #[error("Create API client error")]
    CreateAPIClientError(#[source] anyhow::Error),
    #[error("Empty base or head")]
    EmptyBaseOrHead(String),
    #[error("Base equals head")]
    BaseEqualsHead(String),
    #[error("Api client error")]
    APIClientError(#[source] anyhow::Error),
    #[error("API response is not normal")]
    APIResponseError(String),
    #[error("Cannot parse response json")]
    APIResponseParseError(#[source] anyhow::Error),
    #[error("Cannot got from json")]
    CannotGotFromJsonError(String),
}

#[async_trait]
pub trait GetFirstCommitFromCompare {
    async fn perform(&self, params: FirstCommitFromCompareParams) -> Result<FirstCommitItem, GetFirstCommitFromCompareError>;
}
