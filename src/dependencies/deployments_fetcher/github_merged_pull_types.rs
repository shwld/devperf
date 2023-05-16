use octocrab::Octocrab;

use super::interface::{DeploymentsFetcherError, DeploymentsFetcherParams, RepositoryInfo};
use crate::common_types::github_personal_token::ValidatedGitHubPersonalToken;

pub(super) type GetClient =
    fn(ValidatedGitHubPersonalToken) -> Result<Octocrab, DeploymentsFetcherError>;

// ---------------------------
// Fetching step
// ---------------------------
pub(super) type GitHubMergedPullRequestItem = octocrab::models::pulls::PullRequest;
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)] // most are HerokuRelease
pub(super) enum GitHubMergedPullRequestItemOrRepositoryInfo {
    GitHubMergedPullRequestItem(GitHubMergedPullRequestItem),
    RepositoryInfo(RepositoryInfo),
}

pub(super) trait GitHubMergedPullsFetcher {
    fn fetch(
        &self,
        params: DeploymentsFetcherParams,
    ) -> Result<Vec<GitHubMergedPullRequestItemOrRepositoryInfo>, DeploymentsFetcherError>;
}

// ---------------------------
// Filtering step
// ---------------------------
pub(super) type IsDeployablePullRequest = fn(&GitHubMergedPullRequestItem) -> bool;
