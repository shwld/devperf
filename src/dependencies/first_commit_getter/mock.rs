use async_trait::async_trait;

use crate::shared::datetime_utc::parse;

use super::interface::{
    FirstCommitGetter, FirstCommitGetterError, FirstCommitItem, ValidatedFirstCommitGetterParams,
};

pub struct FirstCommitGetterWithMock {}
#[async_trait]
impl FirstCommitGetter for FirstCommitGetterWithMock {
    async fn get(
        &self,
        _params: ValidatedFirstCommitGetterParams,
    ) -> Result<FirstCommitItem, FirstCommitGetterError> {
        let committed_at = parse("2021-04-10 00:00:00").expect("Could not parse committed_at_str");
        let first_commit = FirstCommitItem {
            sha: "sha".to_string(),
            message: "message".to_string(),
            resource_path: "resource_path".to_string(),
            committed_at,
            creator_login: "creator_login".to_string(),
        };

        Ok(first_commit)
    }
}
