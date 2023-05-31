use async_trait::async_trait;
use octocrab::Octocrab;

use super::{
    github_merged_pull_graphql::MergedPullsPullsNode,
    interface::{DeploymentLog, DeploymentsFetcherError, DeploymentsFetcherParams},
};
use crate::common_types::github_personal_token::ValidatedGitHubPersonalToken;

pub(super) type GetClient =
    fn(ValidatedGitHubPersonalToken) -> Result<Octocrab, DeploymentsFetcherError>;

// ---------------------------
// Fetching step
// ---------------------------

#[async_trait]
pub(super) trait GitHubMergedPullsFetcher {
    async fn fetch(
        &self,
        params: DeploymentsFetcherParams,
    ) -> Result<Vec<MergedPullsPullsNode>, DeploymentsFetcherError>;
}

// ---------------------------
// Collecting step
// ---------------------------
pub(super) type CollectToItems = fn(items: Vec<MergedPullsPullsNode>) -> Vec<DeploymentLog>;
