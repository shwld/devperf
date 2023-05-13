use async_trait::async_trait;
use octocrab::{
    models::{repos::GitUserTime, User},
    Octocrab,
};
use serde::Deserialize;

use super::interface::{
    FirstCommitGetter, FirstCommitGetterError, FirstCommitItem, ValidatedFirstCommitGetterParams,
};
use crate::common_types::{
    github_owner_repo::ValidatedGitHubOwnerRepo,
    github_personal_token::ValidatedGitHubPersonalToken,
};

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
) -> Result<Octocrab, FirstCommitGetterError> {
    let client = Octocrab::builder()
        .personal_token(github_personal_token.to_string())
        .build()
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(FirstCommitGetterError::APIClientError)?;

    Ok(client)
}

async fn fetch_first_commit_from_compare(
    github_personal_token: ValidatedGitHubPersonalToken,
    github_owner_repo: ValidatedGitHubOwnerRepo,
    params: ValidatedFirstCommitGetterParams,
) -> Result<FirstCommitItem, FirstCommitGetterError> {
    let path = format!(
        "https://api.github.com/repos/{owner}/{repo}/compare/{base}...{head}",
        owner = github_owner_repo.get_owner(),
        repo = github_owner_repo.get_repo(),
        base = params.get_base(),
        head = params.get_head()
    );
    let result = get_client(github_personal_token)?
        ._get(path, None::<&()>)
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(FirstCommitGetterError::APIClientError)?;
    let status = result.status();
    if !result.status().is_success() {
        return Err(FirstCommitGetterError::APIResponseError(format!(
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
                params.get_base(),
                params.get_head(),
                e.to_string()
            )
        })
        .map_err(FirstCommitGetterError::APIResponseParseError)?;
    // log::debug!("res: {:?}", res);
    log::debug!(
        "base: {:?}, head: {:?}, results: {:#?}",
        params.get_base(),
        params.get_head(),
        json
    );
    let first_commit =
        json.commits
            .first()
            .ok_or(FirstCommitGetterError::CannotGotFromJsonError(
                "commits".to_string(),
            ))?;
    let committed_at = first_commit
        .clone()
        .commit
        .author
        .and_then(|x| x.date)
        .ok_or(FirstCommitGetterError::CannotGotFromJsonError(
            "date".to_string(),
        ))?;
    let creator_login = first_commit.clone().author.map(|x| x.login).ok_or(
        FirstCommitGetterError::CannotGotFromJsonError("login".to_string()),
    )?;
    Ok(FirstCommitItem {
        sha: first_commit.clone().sha,
        message: first_commit.clone().commit.message,
        resource_path: first_commit.clone().html_url,
        committed_at,
        creator_login,
    })
}

pub struct FirstCommitGetterWithGitHub {
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
}
#[async_trait]
impl FirstCommitGetter for FirstCommitGetterWithGitHub {
    async fn get(
        &self,
        params: ValidatedFirstCommitGetterParams,
    ) -> Result<FirstCommitItem, FirstCommitGetterError> {
        let first_commit = fetch_first_commit_from_compare(
            self.github_personal_token.clone(),
            self.github_owner_repo.clone(),
            params,
        )
        .await?;

        Ok(first_commit)
    }
}
