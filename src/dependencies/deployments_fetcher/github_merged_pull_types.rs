use async_trait::async_trait;
use chrono::{DateTime, Utc};
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

use super::interface::{DeploymentLog, DeploymentsFetcherError, DeploymentsFetcherParams};
use crate::common_types::github_personal_token::ValidatedGitHubPersonalToken;

pub(super) type GetClient =
    fn(ValidatedGitHubPersonalToken) -> Result<Octocrab, DeploymentsFetcherError>;

// ---------------------------
// Fetching step
// ---------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsResponse {
    pub data: MergedPullsData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsData {
    pub repository_owner: MergedPullsRepositoryOwner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsRepositoryOwner {
    pub repository: MergedPullsRepository,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsRepository {
    pub pulls: MergedPullsPulls,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsPulls {
    pub nodes: Vec<MergedPullsPullsNode>,
    pub page_info: MergedPullsPageInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsPullsNode {
    pub id: String,
    pub number: u64,
    pub title: String,
    pub base_ref: Option<MergedPullsBaseRef>,
    pub merged_by: Option<MergedPullsUser>,
    pub merged_at: Option<DateTime<Utc>>,
    pub merge_commit: Option<MergedPullsCommit>,
    pub base_commit_sha: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsBaseRef {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsCommits {
    pub nodes: Vec<MergedPullsCommitsNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsCommitsNode {
    pub id: String,
    pub commit: MergedPullsCommit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsCommit {
    pub id: String,
    pub sha: String,
    pub message: String,
    pub resource_path: String,
    pub committed_date: DateTime<Utc>,
    pub author: Option<MergedPullsCommitAuthor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsCommitAuthor {
    pub user: Option<MergedPullsUser>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsUser {
    pub login: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MergedPullsPageInfo {
    pub end_cursor: Option<String>,
    pub has_next_page: bool,
}

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
