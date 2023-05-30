use chrono::{DateTime, Utc};
use octocrab::models::repos::RepoCommit;

use super::heroku_release_api_response::HerokuReleaseItem;

#[derive(Debug, Clone)]
pub(super) struct GitHubRepositoryInfo {
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)] // most are HerokuRelease
pub(super) enum HerokuReleaseOrRepositoryInfo {
    HerokuRelease(HerokuRelease),
    RepositoryInfo(GitHubRepositoryInfo),
}

#[derive(Debug, Clone)]
pub(super) struct HerokuRelease {
    pub release: HerokuReleaseItem,
    pub commit: RepoCommit,
}
