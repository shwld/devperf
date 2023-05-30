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
pub(super) struct MergedPullsResponse {
    pub(super) data: MergedPullsData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsData {
    pub(super) repository_owner: MergedPullsRepositoryOwner,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsRepositoryOwner {
    pub(super) repository: MergedPullsRepository,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsRepository {
    pub(super) pulls: MergedPullsPulls,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsPulls {
    pub(super) nodes: Vec<MergedPullsPullsNode>,
    pub(super) page_info: MergedPullsPageInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsPullsNode {
    pub(super) id: String,
    pub(super) number: u64,
    pub(super) title: String,
    pub(super) base_ref: Option<MergedPullsBaseRef>,
    pub(super) merged_by: Option<MergedPullsUser>,
    pub(super) merged_at: Option<DateTime<Utc>>,
    pub(super) merge_commit: Option<MergedPullsCommit>,
    pub(super) base_commit_sha: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsBaseRef {
    pub(super) id: String,
    pub(super) name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsCommits {
    pub(super) nodes: Vec<MergedPullsCommitsNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsCommitsNode {
    pub(super) id: String,
    pub(super) commit: MergedPullsCommit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsCommit {
    pub(super) id: String,
    pub(super) sha: String,
    pub(super) message: String,
    pub(super) resource_path: String,
    pub(super) committed_date: DateTime<Utc>,
    pub(super) author: Option<MergedPullsCommitAuthor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsCommitAuthor {
    pub(super) user: Option<MergedPullsUser>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsUser {
    pub(super) login: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct MergedPullsPageInfo {
    pub(super) end_cursor: Option<String>,
    pub(super) has_next_page: bool,
}
