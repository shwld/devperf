use super::{
    github_deployment::{
        DeploymentsDeploymentsNodeGraphQLResponse, DeploymentsDeploymentsStatusNodeGraphQLResponse,
    },
    github_deployment_types::DeploymentNodeGraphQLResponseOrRepositoryInfo,
    interface::{
        BaseCommitShaOrRepositoryInfo, DeploymentInfo, DeploymentLog, DeploymentsFetcher,
        DeploymentsFetcherError, DeploymentsFetcherParams,
    },
};
use crate::{
    common_types::{
        commit::Commit, github_deployment_environment::ValidatedGitHubDeploymentEnvironment,
        github_owner_repo::ValidatedGitHubOwnerRepo,
        github_personal_token::ValidatedGitHubPersonalToken,
    },
    dependencies::deployments_fetcher::{
        github_deployment_graphql::{deployments_query, DeploymentsGraphQLResponse},
        shared::get_created_at,
    },
    shared::non_empty_vec::NonEmptyVec,
};
use anyhow::anyhow;
use async_trait::async_trait;
use octocrab::Octocrab;

fn get_client(
    github_personal_token: ValidatedGitHubPersonalToken,
) -> Result<Octocrab, DeploymentsFetcherError> {
    let client = Octocrab::builder()
        .personal_token(github_personal_token.to_string())
        .build()
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(DeploymentsFetcherError::CreateAPIClientError)?;

    Ok(client)
}

async fn fetch_deployments(
    github_personal_token: ValidatedGitHubPersonalToken,
    github_owner_repo: ValidatedGitHubOwnerRepo,
    environment: ValidatedGitHubDeploymentEnvironment,
) -> Result<Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo>, DeploymentsFetcherError> {
    let mut after: Option<String> = None;
    let mut has_next_page = true;
    let mut deployment_nodes: Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo> = Vec::new();
    let github_client = get_client(github_personal_token.clone())?;

    // 全ページ取得
    while has_next_page {
        let query = deployments_query(github_owner_repo.clone(), environment.clone(), after);

        let results: DeploymentsGraphQLResponse = github_client
            .graphql(&query)
            .await
            .map_err(|e| anyhow!(e))
            .map_err(DeploymentsFetcherError::FetchError)?;
        let new_nodes = results.data.repository_owner.repository.deployments.nodes.into_iter().map(DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse).collect::<Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo>>();
        deployment_nodes = [&deployment_nodes[..], &new_nodes[..]].concat();
        has_next_page = results
            .data
            .repository_owner
            .repository
            .deployments
            .page_info
            .has_next_page;
        after = results
            .data
            .repository_owner
            .repository
            .deployments
            .page_info
            .end_cursor;

        log::debug!("has_next_page: {:#?}", has_next_page);
        // 初回デプロイとリードタイムを比較するためのリポジトリ作成日を取得
        if !has_next_page {
            let repo_created_at =
                get_created_at(github_personal_token.clone(), github_owner_repo.clone())
                    .await
                    .map_err(|e| anyhow!(e))
                    .map_err(DeploymentsFetcherError::GetRepositoryCreatedAtError)?;
            log::debug!("repo_created_at: {:#?}", repo_created_at);
            deployment_nodes.push(
                DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryCreatedAt(repo_created_at),
            );
        }
    }

    Ok(deployment_nodes)
}

fn has_success_status(deployment: &DeploymentNodeGraphQLResponseOrRepositoryInfo) -> bool {
    let deployment = match deployment {
        DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse(deployment) => deployment,
        _ => return true,
    };
    let statuses = deployment
        .statuses
        .nodes
        .iter()
        .map(|x| x.state.to_uppercase())
        .collect::<Vec<String>>();
    let has_success = !statuses.is_empty() && statuses.iter().any(|state| state == "SUCCESS");
    has_success
}

fn find_status(
    deployment: &DeploymentsDeploymentsNodeGraphQLResponse,
) -> Option<DeploymentsDeploymentsStatusNodeGraphQLResponse> {
    let status = deployment
        .statuses
        .nodes
        .iter()
        .find(|&x| x.state.to_uppercase() == "SUCCESS")
        .cloned();

    status
}

fn convert_to_items(
    deployment_nodes: NonEmptyVec<DeploymentNodeGraphQLResponseOrRepositoryInfo>,
) -> Vec<DeploymentLog> {
    let mut sorted: NonEmptyVec<DeploymentNodeGraphQLResponseOrRepositoryInfo> = deployment_nodes;
    sorted.sort_by_key(|a| match a {
        DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse(deployment) => deployment.created_at,
        DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryCreatedAt(created_at) => *created_at,
    });
    let (first_item, rest) = sorted.get();

    // TODO: 無理やりすぎる
    let rest = rest.iter().flat_map(|x| match x {
        DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse(deployment) => Some(deployment.clone()),
        DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryCreatedAt(_) => None,
    }).collect::<Vec<DeploymentsDeploymentsNodeGraphQLResponse>>();

    let first_commit: BaseCommitShaOrRepositoryInfo = match first_item {
        DeploymentNodeGraphQLResponseOrRepositoryInfo::DeploymentsDeploymentsNodeGraphQLResponse(item) => BaseCommitShaOrRepositoryInfo::BaseCommitSha(item.commit.sha),
        DeploymentNodeGraphQLResponseOrRepositoryInfo::RepositoryCreatedAt(created_at) => BaseCommitShaOrRepositoryInfo::RepositoryCreatedAt(created_at),
    };

    let deployment_items = rest
        .iter()
        .scan(
            first_commit,
            |previous: &mut BaseCommitShaOrRepositoryInfo,
             deployment: &DeploymentsDeploymentsNodeGraphQLResponse| {
                let status = find_status(deployment);
                let commit_item = Commit {
                    sha: deployment.clone().commit.sha,
                    message: deployment.clone().commit.message,
                    resource_path: deployment.clone().commit.commit_resource_path,
                    committed_at: deployment.clone().commit.committed_date,
                    creator_login: deployment.clone().creator.login,
                };
                let deployment_item = DeploymentLog {
                    info: DeploymentInfo::GithubDeployment {
                        id: deployment.clone().id,
                    },
                    head_commit: commit_item.clone(),
                    base: previous.clone(),
                    creator_login: deployment.clone().creator.login,
                    deployed_at: status.map_or(deployment.created_at, |x| x.created_at),
                };
                *previous = BaseCommitShaOrRepositoryInfo::BaseCommitSha(commit_item.sha);
                Some(deployment_item)
            },
        )
        .collect::<Vec<DeploymentLog>>();

    deployment_items
}

pub struct DeploymentsFetcherWithGithubDeployment {
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub environment: ValidatedGitHubDeploymentEnvironment,
}
#[async_trait]
impl DeploymentsFetcher for DeploymentsFetcherWithGithubDeployment {
    async fn fetch(
        &self,
        _params: DeploymentsFetcherParams,
    ) -> Result<Vec<DeploymentLog>, DeploymentsFetcherError> {
        let deployment_nodes = fetch_deployments(
            self.github_personal_token.clone(),
            self.github_owner_repo.clone(),
            self.environment.clone(),
        )
        .await?
        .into_iter()
        .filter(has_success_status)
        .collect::<Vec<DeploymentNodeGraphQLResponseOrRepositoryInfo>>();
        let non_empty_nodes = NonEmptyVec::new(deployment_nodes)
            .map_err(|e| anyhow::anyhow!(e))
            .map_err(DeploymentsFetcherError::DeploymentsFetcherResultIsEmptyList)?;
        let deployment_items = convert_to_items(non_empty_nodes);

        Ok(deployment_items)
    }
}
