use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common_types::github_owner_repo::ValidatedGitHubOwnerRepo;

pub(super) fn merged_pulls_query(
    owner_repo: ValidatedGitHubOwnerRepo,
    after: Option<String>,
) -> String {
    let query = format!("
        query {{
          repository_owner: repositoryOwner(login: \"{owner}\") {{
            repository(name: \"{repo}\") {{
              pulls: pullRequests(first: 100, states: [MERGED], orderBy: {{field: CREATED_AT, direction: DESC}}{after}) {{
                nodes {{
                  id
                  number
                  title
                  base_ref: baseRef {{
                    id
                    name
                  }}
                  merged_by: mergedBy {{
                    login
                  }}
                  merged_at: mergedAt
                  merge_commit: mergeCommit {{
                    id
                    sha: oid
                    message
                    resource_path: resourcePath
                    committed_date: committedDate
                    author {{
                      user {{
                        login
                      }}
                    }}
                  }}
                  base_commit_sha: baseRefOid
                }}
                page_info: pageInfo {{
                  end_cursor: endCursor
                  has_next_page: hasNextPage
                }}
              }}
            }}
          }}
        }}
    ", owner = owner_repo.get_owner(), repo = owner_repo.get_repo(), after = after.map_or_else(|| "".to_owned(), |cursor| format!(", after: \"{}\"", cursor)));

    query
}

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
