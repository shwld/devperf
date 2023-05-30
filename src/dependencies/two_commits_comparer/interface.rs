use async_trait::async_trait;
use thiserror::Error;

use crate::common_types::commit::Commit;

#[derive(Debug, Error)]
pub enum ValidatedCommitShaPairError {
    #[error("Empty base or head")]
    EmptyBaseOrHead(String),
    #[error("Base equals head")]
    BaseEqualsHead(String),
}

#[derive(Debug)]
pub struct ValidatedCommitShaPair {
    base: String,
    head: String,
}
impl ValidatedCommitShaPair {
    pub fn new(base: String, head: String) -> Result<Self, ValidatedCommitShaPairError> {
        if base.is_empty() || head.is_empty() {
            return Err(ValidatedCommitShaPairError::EmptyBaseOrHead(format!(
                "base: {:?}, head: {:?}",
                base, head
            )));
        }
        if base == head {
            return Err(ValidatedCommitShaPairError::BaseEqualsHead(format!(
                "base: {:?}, head: {:?}",
                base, head
            )));
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
pub enum TwoCommitsComparerError {
    #[error("Api client error")]
    CannotBuildAPIClient(#[source] anyhow::Error),
    #[error("API response is not normal")]
    InvalidAPIResponse(String),
    #[error("Cannot parse response json")]
    CannotParseResponse(#[source] anyhow::Error),
    #[error("Cannot got from json: {0}")]
    CannotGotFromJson(String),
}

#[async_trait]
pub trait TwoCommitsComparer {
    async fn compare(
        &self,
        commit_sha_pair: ValidatedCommitShaPair,
    ) -> Result<Vec<Commit>, TwoCommitsComparerError>;
}
