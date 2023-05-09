use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::future::try_join_all;
use octocrab::models::repos::RepoCommit;
use serde::{Serialize, Deserialize};
use reqwest::{Client};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use http_cache_reqwest::{Cache, CacheMode, CACacheManager, HttpCache};

use crate::{dependencies::{read_project_config::interface::{ProjectConfig, ResourceConfig}, fetch_deployments::interface::{DeploymentItem, CommitItem}, github_api::GitHubAPI}, common_types::NonEmptyVec};

use super::interface::{FetchDeploymentsError, CommitOrRepositoryInfo, RepositoryInfo, FetchDeployments, FetchDeploymentsParams};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HerokuReleaseItem {
    pub addon_plan_names: Vec<String>,
    pub app: HerokuReleaseApp,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub status: String,
    pub id: String,
    pub slug: Option<HerokuReleaseSlug>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user: HerokuReleaseUser,
    pub version: u64,
    pub current: bool,
    pub output_stream_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HerokuReleaseApp {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HerokuReleaseSlug {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HerokuReleaseUser {
    pub email: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HerokuSlugItem {
    pub blob: HerokuSlugBlobItem,
    pub buildpack_provided_description: String,
    pub checksum: String,
    pub commit: String,
    pub commit_description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub id: String,
    pub size: u64,
    pub stack: HerokuSlugStackItem,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HerokuSlugBlobItem {
    pub method: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HerokuSlugStackItem {
    pub id: String,
    pub name: String,
}

pub fn create_http_client() -> ClientWithMiddleware {
  let client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: None,
        }))
        .build();
  client
}

async fn get_slug(project_config: ProjectConfig, slug_id: &str) -> Result<HerokuSlugItem, FetchDeploymentsError> {
    let resource_config = match project_config.resource {
        ResourceConfig::HerokuRelease(resource) => Ok(resource),
        _ => Err(FetchDeploymentsError::CreateAPIClientError(anyhow::anyhow!("Resource is not HerokuRelease"))),
    }?;
    let client = create_http_client();
    let url = format!("https://api.heroku.com/apps/{app_name}/slugs/{slug_id}", app_name = resource_config.heroku_app_name, slug_id = slug_id);
    let slug = client.get(url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}", token = resource_config.heroku_api_token))
        .header(reqwest::header::ACCEPT, "application/vnd.heroku+json; version=3")
        .send().await.map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::CommitIsNotFound)?
        .json::<HerokuSlugItem>().await.map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::CommitIsNotFound)?;

    Ok(slug)
}

async fn get_commit(github_api: GitHubAPI, project_config: ProjectConfig, sha: &str) -> Result<RepoCommit, FetchDeploymentsError> {
    let resource_config = match project_config.resource {
        ResourceConfig::HerokuRelease(resource) => Ok(resource),
        _ => Err(FetchDeploymentsError::CreateAPIClientError(anyhow::anyhow!("Resource is not HerokuRelease"))),
    }?;
    let github_api_client = github_api.clone().get_client().map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::CreateAPIClientError)?;
    let commit: RepoCommit = github_api_client.get(format!("/repos/{owner}/{repo}/commits/{ref}", owner = resource_config.github_owner, repo = resource_config.github_repo, ref = &sha), None::<&()>).await.map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::CommitIsNotFound)?;

    Ok(commit)
}

#[derive(Debug,Clone)]
struct GitHubRepositoryInfo {
    pub created_at: DateTime<Utc>,
}

#[derive(Debug,Clone)]
enum HerokuReleaseOrRepositoryInfo {
    HerokuRelease(HerokuRelease),
    RepositoryInfo(GitHubRepositoryInfo),
}

#[derive(Debug,Clone)]
struct HerokuRelease {
    pub release: HerokuReleaseItem,
    pub slug: HerokuSlugItem,
    pub commit: RepoCommit,
}

async fn attach_commit(github_api: GitHubAPI, project_config: ProjectConfig, release: HerokuReleaseItem) -> Result<HerokuReleaseOrRepositoryInfo, FetchDeploymentsError> {
    let slug = get_slug(project_config.clone(), &release.slug.clone().unwrap().id).await?;
    let commit = get_commit(github_api.clone(), project_config.clone(), &slug.commit).await?;

    Ok(HerokuReleaseOrRepositoryInfo::HerokuRelease(HerokuRelease {
        release,
        slug,
        commit,
    }))
}

async fn get_created_at(github_api: GitHubAPI, owner: &str, repo: &str) -> Result<chrono::DateTime<chrono::Utc>, FetchDeploymentsError> {
    let github_api_client = github_api.clone().get_client().map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::CreateAPIClientError)?;
    let result = github_api_client.repos(owner, repo).get().await.map(|r| r.created_at).map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::GetRepositoryCreatedAtError)?;
    let created_at = result.ok_or(FetchDeploymentsError::RepositoryNotFound(format!("{}/{}", owner.to_owned(), repo.to_owned())))?;

    Ok(created_at)
}


fn convert_to_items(deployment_nodes: NonEmptyVec<HerokuReleaseOrRepositoryInfo>) -> Vec<DeploymentItem> {
    let mut sorted: NonEmptyVec<HerokuReleaseOrRepositoryInfo> = deployment_nodes.clone();
    sorted.sort_by_key(|a| match a {
        HerokuReleaseOrRepositoryInfo::HerokuRelease(release) => release.release.created_at,
        HerokuReleaseOrRepositoryInfo::RepositoryInfo(info) => info.created_at,
    });
    let (first_item, rest) = sorted.get();

    // TODO: 無理やりすぎる
    let rest = rest.iter().flat_map(|x| match x {
        HerokuReleaseOrRepositoryInfo::HerokuRelease(release) => Some(release.clone()),
        HerokuReleaseOrRepositoryInfo::RepositoryInfo(info) => None,
    }).collect::<Vec<HerokuRelease>>();

    let first_commit: CommitOrRepositoryInfo = match first_item {
        HerokuReleaseOrRepositoryInfo::HerokuRelease(item) => CommitOrRepositoryInfo::Commit(CommitItem {
            sha: item.release.id.clone(),
            message: item.commit.commit.message.clone(),
            resource_path: item.commit.html_url.clone(),
            committed_at: item.commit.commit.author.map(|x| x.date.unwrap()).unwrap(), // TODO unwrap
            creator_login: item.commit.author.map(|x| x.login.clone()).unwrap(), // TODO unwrap
        }),
        HerokuReleaseOrRepositoryInfo::RepositoryInfo(info) => CommitOrRepositoryInfo::RepositoryInfo(RepositoryInfo {
            created_at: info.created_at,
        }),
    };

    let deployment_items = rest
        .iter()
        .scan(first_commit, |previous: &mut CommitOrRepositoryInfo, release: &HerokuRelease| {
            let commit_item = CommitItem {
                sha: release.clone().commit.sha,
                message: release.clone().commit.commit.message,
                resource_path: release.clone().commit.html_url,
                committed_at: release.clone().commit.commit.author.map(|x| x.date.unwrap()).unwrap(), // TODO unwrap
                creator_login: release.clone().commit.author.map(|x| x.login).unwrap(), // TODO unwrap
            };
            let deployment_item = DeploymentItem {
                id: release.clone().release.id,
                head_commit: commit_item.clone(),
                base: previous.clone(),
                creator_login: release.clone().commit.author.map(|x| x.login).unwrap(), // TODO unwrap
                deployed_at: release.release.created_at,
            };
            *previous = CommitOrRepositoryInfo::Commit(commit_item);
            Some(deployment_item)
        }).collect::<Vec<DeploymentItem>>();

    deployment_items
}


// FIXME: remove depends modules::github
pub async fn list(github_api: GitHubAPI, project_config: ProjectConfig) -> Result<Vec<DeploymentItem>, FetchDeploymentsError> {
    let resource_config = match project_config.clone().resource {
        ResourceConfig::HerokuRelease(resource) => Ok(resource),
        _ => Err(FetchDeploymentsError::CreateAPIClientError(anyhow::anyhow!("Resource is not HerokuRelease"))),
    }?;
    let client = create_http_client();
    let url = format!("https://api.heroku.com/apps/{app_name}/releases", app_name = resource_config.heroku_app_name);
    let releases: Vec<HerokuReleaseItem> = client.get(url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}", token = resource_config.heroku_api_token))
        .header(reqwest::header::ACCEPT, "application/vnd.heroku+json; version=3")
        .header(reqwest::header::RANGE, "version ..; order=desc;")
        .send().await.map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::FetchDeploymentsError)?
        .json::<Vec<HerokuReleaseItem>>().await.map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::FetchDeploymentsError)?;

    let mut deployments = try_join_all(releases.iter().map(|release| {
        attach_commit(github_api.clone(), project_config.clone(), release.clone())
    })).await?;
    let repo_creatd_at = get_created_at(github_api.clone(), &resource_config.github_owner, &resource_config.github_repo).await.map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::GetRepositoryCreatedAtError)?;
    log::debug!("repo_creatd_at: {:#?}", repo_creatd_at);
    deployments.push(HerokuReleaseOrRepositoryInfo::RepositoryInfo(GitHubRepositoryInfo { created_at: repo_creatd_at }));
    let non_empty_nodes = NonEmptyVec::new(deployments)
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(FetchDeploymentsError::FetchDeploymentsResultIsEmptyList)?;

    let deployment_items = convert_to_items(non_empty_nodes);

    Ok(deployment_items)
}

pub struct FetchDeploymentsWithHerokuRelease {
    pub github_api: GitHubAPI,
    pub project_config: ProjectConfig,
}
#[async_trait]
impl FetchDeployments for FetchDeploymentsWithHerokuRelease {
    // TODO: paramsとproject_configどっちかにしろ
    async fn perform(&self, _params: FetchDeploymentsParams) -> Result<Vec<DeploymentItem>, FetchDeploymentsError> {
        let resource_config = match self.project_config.clone().resource {
            ResourceConfig::HerokuRelease(resource) => Ok(resource),
            _ => Err(FetchDeploymentsError::CreateAPIClientError(anyhow::anyhow!("Resource is not HerokuRelease"))),
        }?;
        let client = create_http_client();
        let url = format!("https://api.heroku.com/apps/{app_name}/releases", app_name = resource_config.heroku_app_name);
        let releases: Vec<HerokuReleaseItem> = client.get(url)
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}", token = resource_config.heroku_api_token))
            .header(reqwest::header::ACCEPT, "application/vnd.heroku+json; version=3")
            .header(reqwest::header::RANGE, "version ..; order=desc;")
            .send().await.map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::FetchDeploymentsError)?
            .json::<Vec<HerokuReleaseItem>>().await.map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::FetchDeploymentsError)?;

        let mut deployments = try_join_all(releases.iter().map(|release| {
            attach_commit(self.github_api.clone(), self.project_config.clone(), release.clone())
        })).await?;
        let repo_creatd_at = get_created_at(self.github_api.clone(), &resource_config.github_owner, &resource_config.github_repo).await.map_err(|e| anyhow::anyhow!(e)).map_err(FetchDeploymentsError::GetRepositoryCreatedAtError)?;
        log::debug!("repo_creatd_at: {:#?}", repo_creatd_at);
        deployments.push(HerokuReleaseOrRepositoryInfo::RepositoryInfo(GitHubRepositoryInfo { created_at: repo_creatd_at }));
        let non_empty_nodes = NonEmptyVec::new(deployments)
            .map_err(|e| anyhow::anyhow!(e))
            .map_err(FetchDeploymentsError::FetchDeploymentsResultIsEmptyList)?;

        let deployment_items = convert_to_items(non_empty_nodes);

        Ok(deployment_items)
    }
}
