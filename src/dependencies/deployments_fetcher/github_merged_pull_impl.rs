use async_trait::async_trait;
use octocrab::Octocrab;
use wildmatch::WildMatch;

use crate::{
    common_types::{
        deploy_branch_name::ValidatedDeployBranchName, github_owner_repo::ValidatedGitHubOwnerRepo,
        github_personal_token::ValidatedGitHubPersonalToken,
    },
    dependencies::deployments_fetcher::github_merged_pull_types::MergedPullsResponse,
};

use super::{
    github_merged_pull_types::{
        CollectToItems, GetClient, GitHubMergedPullsFetcher, MergedPullsPullsNode,
    },
    interface::{
        BaseCommitShaOrRepositoryInfo, CommitItem, DeploymentInfo, DeploymentItem,
        DeploymentsFetcher, DeploymentsFetcherError, DeploymentsFetcherParams,
    },
};

fn merged_pulls_query(owner_repo: ValidatedGitHubOwnerRepo, after: Option<String>) -> String {
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
                            .map(|it| it >= params.since && it <= params.until)
                            .unwrap_or(false)
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

const collect_to_items: CollectToItems = |items: Vec<MergedPullsPullsNode>| -> Vec<DeploymentItem> {
    items
        .into_iter()
        .map(|item| {
            let head_commit = item
                .merge_commit
                .map(|node| CommitItem {
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
            let deployed_at = item
                .merged_at
                .ok_or(DeploymentsFetcherError::InvalidResponse(
                    "merged_at is empty".to_string(),
                ));
            if let (Ok(head_commit), Ok(deployed_at)) = (head_commit, deployed_at) {
                Ok(DeploymentItem {
                    info: DeploymentInfo::GithubMergedPullRequest {
                        id: item.id,
                        number: item.number,
                        title: item.title,
                    },
                    head_commit,
                    base: BaseCommitShaOrRepositoryInfo::BaseCommitSha(item.base_commit_sha),
                    creator_login: item
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
    ) -> Result<Vec<DeploymentItem>, DeploymentsFetcherError> {
        let fetcher = GitHubMergedPullsFetcherImpl {
            github_personal_token: self.github_personal_token.clone(),
            github_owner_repo: self.github_owner_repo.clone(),
            deploy_trigger_branch: self.deploy_trigger_branch.clone(),
        };
        let deployment_items = fetcher.fetch(params).await?;
        let colleted_items = collect_to_items(deployment_items);

        Ok(colleted_items)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{
        common_types::{
            deploy_branch_name::ValidatedDeployBranchName,
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
                let result = fetcher
                    .fetch(DeploymentsFetcherParams {
                        since: datetime_utc::parse("2023-05-01").unwrap(),
                        until: datetime_utc::parse("2023-06-01").unwrap(),
                    })
                    .await;
                println!("{:#?}", result);
                assert_eq!(result.is_ok(), true);
            }
            Err(_e) => {
                println!("GITHUB_PERSONAL_TOKEN is not set");
            }
        }
    }
}
