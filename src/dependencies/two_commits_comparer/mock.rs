use async_trait::async_trait;

use crate::common_types::commit::Commit;

use super::interface::{TwoCommitsComparer, TwoCommitsComparerError, ValidatedCommitShaPair};

pub struct TwoCommitsComparerWithMock {
    pub commits: Vec<Commit>,
}
#[async_trait]
impl TwoCommitsComparer for TwoCommitsComparerWithMock {
    async fn compare(
        &self,
        _commit_sha_pair: ValidatedCommitShaPair,
    ) -> Result<Vec<Commit>, TwoCommitsComparerError> {
        Ok(self.commits.clone())
    }
}
