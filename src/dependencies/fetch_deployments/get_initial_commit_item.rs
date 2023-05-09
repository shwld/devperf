use async_std::task::block_on;
use thiserror::Error;
use crate::dependencies::github_api::GitHubAPI;

use super::interface::CommitItem;


#[derive(Debug, Error)]
pub enum GetInitialDeploymentItemError {
    #[error("Create API client error")]
    CreateAPIClientError(#[source] anyhow::Error),
    #[error("Cannot read the config file")]
    FetchDeploymentsError(#[source] octocrab::Error),
    #[error("Cannot get the first commit")]
    FirstCommitIsRequired(String),
    #[error("Cannot list commit")]
    ListCommitsError(#[source] octocrab::Error),
    #[error("Cannot get the first commit")]
    FirstCommitIsNotFound(String),
    #[error("Cannot get the author")]
    AuthorIsNotFound(String),
}


pub async fn get_initial_commit_item(github_api: GitHubAPI, owner: &str, repo: &str) -> Result<CommitItem, GetInitialDeploymentItemError> {
    let github_api_client = github_api.clone().get_client().map_err(|e| anyhow::anyhow!(e)).map_err(GetInitialDeploymentItemError::CreateAPIClientError)?;
    let query = format!("repo:{owner}/{repo}", owner = owner, repo = repo);
    let page_commit = github_api_client
        .search()
        .commits(&query)
        .sort("committer-date")
        .order("asc")
        .per_page(1)
        .send()
        .await
        .map_err(GetInitialDeploymentItemError::FetchDeploymentsError)?;
    let oldest_commit_sha = page_commit
        .items
        .first()
        .and_then(|c| c.sha.clone())
        .ok_or(GetInitialDeploymentItemError::FirstCommitIsRequired("First commit is required".to_string()))?;
    let commits = block_on(async {
        github_api_client
            .repos(owner, repo)
            .list_commits()
            .sha(oldest_commit_sha)
            .per_page(1)
            .send()
            .await
            .map_err(GetInitialDeploymentItemError::ListCommitsError)
      })?;
    let first_commit = commits.items.first().ok_or(GetInitialDeploymentItemError::FirstCommitIsNotFound("First commit is required".to_string()))?;

    let committed_at = first_commit.clone().commit.author.and_then(|author| author.date);
    let author_login = first_commit.clone().author.map(|author| author.login);

    if let (Some(committed_at), Some(login)) = (committed_at, author_login) {
        Ok(CommitItem {
            sha: first_commit.clone().sha,
            message: first_commit.clone().commit.message,
            resource_path: first_commit.clone().html_url,
            committed_at: committed_at,
            creator_login: login,
        })
    } else {
        Err(GetInitialDeploymentItemError::AuthorIsNotFound("First commit is required".to_string()))
    }
}
