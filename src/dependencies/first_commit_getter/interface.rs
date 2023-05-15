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

#[derive(Debug, Error)]
pub enum ValidatedFirstCommitGetterParamsError {
    #[error("Empty base or head")]
    EmptyBaseOrHead(String),
    #[error("Base equals head")]
    BaseEqualsHead(String),
}

#[derive(Debug)]
pub struct ValidatedFirstCommitGetterParams {
    base: String,
    head: String,
}
impl ValidatedFirstCommitGetterParams {
    pub fn new(base: String, head: String) -> Result<Self, ValidatedFirstCommitGetterParamsError> {
        if base.is_empty() || head.is_empty() {
            return Err(ValidatedFirstCommitGetterParamsError::EmptyBaseOrHead(
                format!("base: {:?}, head: {:?}", base, head),
            ));
        }
        if base == head {
            return Err(ValidatedFirstCommitGetterParamsError::BaseEqualsHead(
                format!("base: {:?}, head: {:?}", base, head),
            ));
        }
        Ok(Self { base, head })
    }
    pub fn get_base(&self) -> String {
        self.base.clone()
    }
    pub fn get_head(&self) -> String {
        self.head.clone()
    }
}

#[derive(Debug, Error)]
pub enum FirstCommitGetterError {
    #[error("Api client error")]
    CannotBuildAPIClient(#[source] anyhow::Error),
    #[error("API response is not normal")]
    InvalidAPIResponse(String),
    #[error("Cannot parse response json")]
    CannotParseResponse(#[source] anyhow::Error),
    #[error("Cannot got from json")]
    CannotGotFromJson(String),
}

#[async_trait]
pub trait FirstCommitGetter {
    async fn get(
        &self,
        params: ValidatedFirstCommitGetterParams,
    ) -> Result<FirstCommitItem, FirstCommitGetterError>;
}
