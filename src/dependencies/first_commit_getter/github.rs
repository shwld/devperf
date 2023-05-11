use async_trait::async_trait;
use chrono::{DateTime, Utc};
use octocrab::Octocrab;

use crate::project_parameter_validating::{
    validate_github_owner_repo::ValidatedGitHubOwnerRepo,
    validate_github_personal_token::ValidatedGitHubPersonalToken,
};

use super::interface::{
    FirstCommitGetter, FirstCommitGetterError, FirstCommitGetterParams, FirstCommitItem,
};

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

pub async fn get_first_commit_from_compare(
    github_personal_token: ValidatedGitHubPersonalToken,
    github_owner_repo: ValidatedGitHubOwnerRepo,
    params: &FirstCommitGetterParams,
) -> Result<FirstCommitItem, FirstCommitGetterError> {
    log::debug!("params: {:?}", params);
    if params.base.is_empty() || params.head.is_empty() {
        return Err(FirstCommitGetterError::EmptyBaseOrHead(format!(
            "base: {:?}, head: {:?}",
            params.base, params.head
        )));
    }
    if params.base == params.head {
        return Err(FirstCommitGetterError::BaseEqualsHead(format!(
            "base: {:?}, head: {:?}",
            params.base, params.head
        )));
    }
    let path = format!(
        "https://api.github.com/repos/{owner}/{repo}/compare/{base}...{head}",
        owner = github_owner_repo.get_owner(),
        repo = github_owner_repo.get_repo(),
        base = params.base,
        head = params.head
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
        .json::<serde_json::Value>()
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(FirstCommitGetterError::APIResponseParseError)?;
    // log::debug!("res: {:?}", res);
    log::debug!("base: {:?}, head: {:?}", params.base, params.head);
    let first_commit = json.get("commits").and_then(|x| x.get(0)).ok_or(
        FirstCommitGetterError::CannotGotFromJsonError("commits".to_string()),
    )?;

    log::debug!("first_commit_result: {:#?}", first_commit);
    let sha =
        first_commit["sha"]
            .as_str()
            .ok_or(FirstCommitGetterError::CannotGotFromJsonError(
                "sha".to_string(),
            ))?;
    let message = first_commit["commit"]["message"].as_str().ok_or(
        FirstCommitGetterError::CannotGotFromJsonError("message".to_string()),
    )?;
    let html_url =
        first_commit["html_url"]
            .as_str()
            .ok_or(FirstCommitGetterError::CannotGotFromJsonError(
                "html_url".to_string(),
            ))?;
    let committed_at = first_commit["commit"]["author"]["date"]
        .as_str()
        .ok_or(FirstCommitGetterError::CannotGotFromJsonError(
            "date".to_string(),
        ))
        .and_then(|date_str| {
            DateTime::parse_from_rfc3339(date_str)
                .map_err(|e| anyhow::anyhow!(e))
                .map_err(FirstCommitGetterError::APIResponseParseError)
        })?
        .with_timezone(&Utc);
    let creator_login = first_commit["author"]["login"].as_str().ok_or(
        FirstCommitGetterError::CannotGotFromJsonError("login".to_string()),
    )?;
    Ok(FirstCommitItem {
        sha: sha.to_string(),
        message: message.to_string(),
        resource_path: html_url.to_string(),
        committed_at,
        creator_login: creator_login.to_string(),
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
        params: FirstCommitGetterParams,
    ) -> Result<FirstCommitItem, FirstCommitGetterError> {
        let first_commit = get_first_commit_from_compare(
            self.github_personal_token.clone(),
            self.github_owner_repo.clone(),
            &params,
        )
        .await?;
        Ok(first_commit)
    }
}
