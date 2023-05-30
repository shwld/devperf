use async_trait::async_trait;
use octocrab::Octocrab;
use wildmatch::WildMatch;

use crate::{
    common_types::{
        commit::Commit, deploy_branch_name::ValidatedDeployBranchName,
        github_owner_repo::ValidatedGitHubOwnerRepo,
        github_personal_token::ValidatedGitHubPersonalToken,
    },
    dependencies::deployments_fetcher::github_merged_pull_types::MergedPullsResponse,
};

use super::{
    github_merged_pull_graphql::merged_pulls_query,
    github_merged_pull_types::{
        CollectToItems, GetClient, GitHubMergedPullsFetcher, MergedPullsPullsNode,
    },
    interface::{
        BaseCommitShaOrRepositoryInfo, DeploymentInfo, DeploymentLog, DeploymentsFetcher,
        DeploymentsFetcherError, DeploymentsFetcherParams,
    },
};

const get_client: GetClient = |
    github_personal_token: ValidatedGitHubPersonalToken,
| -> Result<Octocrab, DeploymentsFetcherError> {
    let client = Octocrab::builder()
        .personal_token(github_personal_token.to_string())
        .build()
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(DeploymentsFetcherError::CreateAPIClientError)?;

    Ok(client)
};

struct GitHubMergedPullsFetcherImpl {
    github_personal_token: ValidatedGitHubPersonalToken,
    github_owner_repo: ValidatedGitHubOwnerRepo,
    deploy_trigger_branch: ValidatedDeployBranchName,
}
#[async_trait]
impl GitHubMergedPullsFetcher for GitHubMergedPullsFetcherImpl {
    async fn fetch(
        &self,
        params: DeploymentsFetcherParams,
    ) -> Result<Vec<MergedPullsPullsNode>, DeploymentsFetcherError> {
        let mut after: Option<String> = None;
        let mut has_next_page = true;
        let mut items: Vec<MergedPullsPullsNode> = Vec::new();
        let github_client = get_client(self.github_personal_token.clone())?;

        // 全ページ取得
        while has_next_page {
            let query = merged_pulls_query(self.github_owner_repo.clone(), after);

            let results: MergedPullsResponse = github_client
                .graphql(&query)
                .await
                .map_err(|e| anyhow::anyhow!(e))
                .map_err(DeploymentsFetcherError::FetchError)?;
            let new_nodes = results
                .data
                .repository_owner
                .repository
                .pulls
                .nodes
                .iter()
                .filter(|it| {
                    let branch_ok = if let Some(base_ref) = &it.base_ref {
                        let wild_match = WildMatch::new(&self.deploy_trigger_branch.to_string());
                        wild_match.matches(&base_ref.name)
                    } else {
                        false
                    };
                    branch_ok
                        && it
                            .merged_at
                            .map_or(false, |merged_at| params.timeframe.is_include(&merged_at))
                })
                .cloned()
                .collect::<Vec<MergedPullsPullsNode>>();
            items = [&items[..], &new_nodes[..]].concat();
            has_next_page = results
                .data
                .repository_owner
                .repository
                .pulls
                .page_info
                .has_next_page;
            after = results
                .data
                .repository_owner
                .repository
                .pulls
                .page_info
                .end_cursor;
        }

        Ok(items)
    }
}

const collect_to_logs: CollectToItems = |nodes: Vec<MergedPullsPullsNode>| -> Vec<DeploymentLog> {
    nodes
        .into_iter()
        .map(|node| {
            let head_commit = node
                .merge_commit
                .map(|node| Commit {
                    sha: node.sha,
                    message: node.message,
                    resource_path: node.resource_path,
                    committed_at: node.committed_date,
                    creator_login: node
                        .author
                        .and_then(|x| x.user)
                        .map(|x| x.login)
                        .unwrap_or_else(|| "".to_string()),
                })
                .ok_or(DeploymentsFetcherError::InvalidResponse(
                    "merge commit is empty".to_string(),
                ));
            let deployed_at = node
                .merged_at
                .ok_or(DeploymentsFetcherError::InvalidResponse(
                    "merged_at is empty".to_string(),
                ));
            if let (Ok(head_commit), Ok(deployed_at)) = (head_commit, deployed_at) {
                Ok(DeploymentLog {
                    info: DeploymentInfo::GithubMergedPullRequest {
                        id: node.id,
                        number: node.number,
                        title: node.title,
                    },
                    head_commit,
                    base: BaseCommitShaOrRepositoryInfo::BaseCommitSha(node.base_commit_sha),
                    creator_login: node
                        .merged_by
                        .map(|x| x.login)
                        .unwrap_or_else(|| "".to_string()),
                    deployed_at,
                })
            } else {
                Err(DeploymentsFetcherError::InvalidResponse(
                    "Invalid response".to_string(),
                ))
            }
        })
        .filter_map(Result::ok)
        .collect()
};

pub struct DeploymentsFetcherWithGithubMergedPullRequest {
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub deploy_trigger_branch: ValidatedDeployBranchName,
}
#[async_trait]
impl DeploymentsFetcher for DeploymentsFetcherWithGithubMergedPullRequest {
    async fn fetch(
        &self,
        params: DeploymentsFetcherParams,
    ) -> Result<Vec<DeploymentLog>, DeploymentsFetcherError> {
        let fetcher = GitHubMergedPullsFetcherImpl {
            github_personal_token: self.github_personal_token.clone(),
            github_owner_repo: self.github_owner_repo.clone(),
            deploy_trigger_branch: self.deploy_trigger_branch.clone(),
        };
        let deployment_logs = fetcher.fetch(params).await?;
        let colleted_items = collect_to_logs(deployment_logs);

        Ok(colleted_items)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{
        common_types::{
            date_time_range::DateTimeRange, deploy_branch_name::ValidatedDeployBranchName,
            github_owner_repo::ValidatedGitHubOwnerRepo,
            github_personal_token::ValidatedGitHubPersonalToken,
        },
        dependencies::deployments_fetcher::{
            github_merged_pull_impl::DeploymentsFetcherWithGithubMergedPullRequest,
            interface::{DeploymentsFetcher, DeploymentsFetcherParams},
        },
        shared::datetime_utc,
    };

    #[tokio::test]
    async fn test() {
        match env::var("GITHUB_PERSONAL_TOKEN") {
            Ok(token) => {
                let token = token.parse::<String>().unwrap();
                let token = ValidatedGitHubPersonalToken::new(Some(token)).unwrap();
                let owner_repo =
                    ValidatedGitHubOwnerRepo::new("shwld/revelup-note".to_string()).unwrap();
                let branch_name = ValidatedDeployBranchName::new(Some("main".to_string())).unwrap();
                let fetcher = DeploymentsFetcherWithGithubMergedPullRequest {
                    github_personal_token: token,
                    github_owner_repo: owner_repo,
                    deploy_trigger_branch: branch_name,
                };
                let timeframe = DateTimeRange::new(
                    datetime_utc::parse("2021-05-01").unwrap(),
                    datetime_utc::parse("2021-06-01").unwrap(),
                )
                .unwrap();
                let result = fetcher.fetch(DeploymentsFetcherParams { timeframe }).await;
                println!("{:#?}", result);
                assert!(result.is_ok());
            }
            Err(_e) => {
                println!("GITHUB_PERSONAL_TOKEN is not set");
            }
        }
    }
}
