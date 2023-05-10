use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::dependencies::github_api::GitHubAPI;

use super::interface::{
    FirstCommitFromCompareParams, FirstCommitItem, GetFirstCommitFromCompare,
    GetFirstCommitFromCompareError,
};

pub async fn get_first_commit_from_compare(
    github_api: GitHubAPI,
    params: &FirstCommitFromCompareParams,
) -> Result<FirstCommitItem, GetFirstCommitFromCompareError> {
    let github_api_client = github_api
        .get_client()
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(GetFirstCommitFromCompareError::CreateAPIClientError)?;
    log::debug!("params: {:?}", params);
    if params.base.is_empty() || params.head.is_empty() {
        return Err(GetFirstCommitFromCompareError::EmptyBaseOrHead(format!(
            "base: {:?}, head: {:?}",
            params.base, params.head
        )));
    }
    if params.base == params.head {
        return Err(GetFirstCommitFromCompareError::BaseEqualsHead(format!(
            "base: {:?}, head: {:?}",
            params.base, params.head
        )));
    }
    let path = format!(
        "https://api.github.com/repos/{owner}/{repo}/compare/{base}...{head}",
        owner = params.owner,
        repo = params.repo,
        base = params.base,
        head = params.head
    );
    let result = github_api_client
        ._get(path, None::<&()>)
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(GetFirstCommitFromCompareError::APIClientError)?;
    let status = result.status();
    if !result.status().is_success() {
        return Err(GetFirstCommitFromCompareError::APIResponseError(format!(
            "status: {:?}",
            status
        )));
    }
    let json = result
        .json::<serde_json::Value>()
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(GetFirstCommitFromCompareError::APIResponseParseError)?;
    // log::debug!("res: {:?}", res);
    log::debug!("base: {:?}, head: {:?}", params.base, params.head);
    let first_commit = json.get("commits").and_then(|x| x.get(0)).ok_or(
        GetFirstCommitFromCompareError::CannotGotFromJsonError("commits".to_string()),
    )?;

    log::debug!("first_commit_result: {:#?}", first_commit);
    let sha = first_commit["sha"].as_str().ok_or(
        GetFirstCommitFromCompareError::CannotGotFromJsonError("sha".to_string()),
    )?;
    let message = first_commit["commit"]["message"].as_str().ok_or(
        GetFirstCommitFromCompareError::CannotGotFromJsonError("message".to_string()),
    )?;
    let html_url = first_commit["html_url"].as_str().ok_or(
        GetFirstCommitFromCompareError::CannotGotFromJsonError("html_url".to_string()),
    )?;
    let committed_at = first_commit["commit"]["author"]["date"]
        .as_str()
        .ok_or(GetFirstCommitFromCompareError::CannotGotFromJsonError(
            "date".to_string(),
        ))
        .and_then(|date_str| {
            DateTime::parse_from_rfc3339(date_str)
                .map_err(|e| anyhow::anyhow!(e))
                .map_err(GetFirstCommitFromCompareError::APIResponseParseError)
        })?
        .with_timezone(&Utc);
    let creator_login = first_commit["author"]["login"].as_str().ok_or(
        GetFirstCommitFromCompareError::CannotGotFromJsonError("login".to_string()),
    )?;
    Ok(FirstCommitItem {
        sha: sha.to_string(),
        message: message.to_string(),
        resource_path: html_url.to_string(),
        committed_at,
        creator_login: creator_login.to_string(),
    })
}

pub struct GetFirstCommitFromCompareWithGitHub {
    pub github_api: GitHubAPI,
}
#[async_trait]
impl GetFirstCommitFromCompare for GetFirstCommitFromCompareWithGitHub {
    async fn perform(
        &self,
        params: FirstCommitFromCompareParams,
    ) -> Result<FirstCommitItem, GetFirstCommitFromCompareError> {
        let first_commit = get_first_commit_from_compare(self.github_api.clone(), &params).await?;
        Ok(first_commit)
    }
}
