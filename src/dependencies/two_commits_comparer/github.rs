use async_trait::async_trait;
use octocrab::{
    models::{repos::GitUserTime, User},
    Octocrab,
};
use serde::Deserialize;

use crate::common_types::{
    commit::Commit, github_owner_repo::ValidatedGitHubOwnerRepo,
    github_personal_token::ValidatedGitHubPersonalToken,
};

use super::interface::{TwoCommitsComparer, TwoCommitsComparerError, ValidatedCommitShaPair};

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct CommitPage {
    pub author: Option<GitUserTime>,
    pub comitter: Option<GitUserTime>,
    pub message: String,
    pub comment_count: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct CommitItem {
    pub url: String,
    pub sha: String,
    pub node_id: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: CommitPage,
    pub author: Option<User>,
    pub committer: Option<User>,
}

#[derive(Deserialize, Debug)]
struct CompareResult {
    pub commits: Vec<CommitItem>,
}

fn get_client(
    github_personal_token: ValidatedGitHubPersonalToken,
) -> Result<Octocrab, TwoCommitsComparerError> {
    let client = Octocrab::builder()
        .personal_token(github_personal_token.to_string())
        .build()
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(TwoCommitsComparerError::CannotBuildAPIClient)?;

    Ok(client)
}

async fn compare_two_commits(
    github_personal_token: ValidatedGitHubPersonalToken,
    github_owner_repo: ValidatedGitHubOwnerRepo,
    commit_sha_pair: ValidatedCommitShaPair,
) -> Result<Vec<Commit>, TwoCommitsComparerError> {
    let path = format!(
        "https://api.github.com/repos/{owner}/{repo}/compare/{base}...{head}",
        owner = github_owner_repo.get_owner(),
        repo = github_owner_repo.get_repo(),
        base = commit_sha_pair.get_base(),
        head = commit_sha_pair.get_head()
    );
    let result = get_client(github_personal_token)?
        ._get(path, None::<&()>)
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(TwoCommitsComparerError::CannotBuildAPIClient)?;
    let status = result.status();
    if !result.status().is_success() {
        return Err(TwoCommitsComparerError::InvalidAPIResponse(format!(
            "status: {:?}",
            status
        )));
    }
    let json = result
        .json::<CompareResult>()
        .await
        // .map_err(|e| anyhow::anyhow!(e))
        .map_err(|e| {
            anyhow::anyhow!(
                "base: {:?}, head: {:?}, error: {:#?}",
                commit_sha_pair.get_base(),
                commit_sha_pair.get_head(),
                e.to_string()
            )
        })
        .map_err(TwoCommitsComparerError::CannotParseResponse)?;
    let commits = json
        .commits
        .into_iter()
        .flat_map(|commit| {
            let committed_at = commit.clone().commit.author.and_then(|x| x.date).ok_or(
                TwoCommitsComparerError::CannotGotFromJson("date".to_string()),
            );
            match committed_at {
                Ok(committed_at) => Ok(Commit {
                    sha: commit.clone().sha,
                    message: commit.clone().commit.message,
                    resource_path: commit.clone().html_url,
                    committed_at,
                    creator_login: commit.author.map(|x| x.login).unwrap_or("".to_string()),
                }),
                Err(e) => Err(e),
            }
        })
        .collect();
    Ok(commits)
}

pub struct TwoCommitsComparerWithGitHub {
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
}
#[async_trait]
impl TwoCommitsComparer for TwoCommitsComparerWithGitHub {
    async fn compare(
        &self,
        commit_sha_pair: ValidatedCommitShaPair,
    ) -> Result<Vec<Commit>, TwoCommitsComparerError> {
        let commits = compare_two_commits(
            self.github_personal_token.clone(),
            self.github_owner_repo.clone(),
            commit_sha_pair,
        )
        .await?;

        Ok(commits)
    }
}
