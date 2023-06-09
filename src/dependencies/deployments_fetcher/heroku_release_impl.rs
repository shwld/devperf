use async_trait::async_trait;
use futures::future::try_join_all;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use octocrab::{models::repos::RepoCommit, Octocrab};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

use super::{
    heroku_release_api_response::{HerokuReleaseItem, HerokuSlugItem},
    heroku_release_types::{HerokuRelease, HerokuReleaseOrRepositoryInfo},
    interface::{
        BaseCommitShaOrRepositoryInfo, DeploymentsFetcher, DeploymentsFetcherError,
        DeploymentsFetcherParams,
    },
};
use crate::{
    common_types::{
        commit::Commit, github_owner_repo::ValidatedGitHubOwnerRepo,
        github_personal_token::ValidatedGitHubPersonalToken,
        heroku_app_name::ValidatedHerokuAppName, heroku_auth_token::ValidatedHerokuAuthToken,
    },
    dependencies::deployments_fetcher::{
        heroku_release_types::GitHubRepositoryInfo,
        interface::{DeploymentInfo, DeploymentLog},
        shared::get_created_at,
    },
    shared::non_empty_vec::NonEmptyVec,
};

pub fn create_http_client() -> ClientWithMiddleware {
    ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: None,
        }))
        .build()
}

async fn get_slug(
    heroku_app_name: ValidatedHerokuAppName,
    heroku_auth_token: ValidatedHerokuAuthToken,
    slug_id: &str,
) -> Result<HerokuSlugItem, DeploymentsFetcherError> {
    let client = create_http_client();
    let url = format!(
        "https://api.heroku.com/apps/{app_name}/slugs/{slug_id}",
        app_name = heroku_app_name,
        slug_id = slug_id
    );
    let slug = client
        .get(url)
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}", token = heroku_auth_token),
        )
        .header(
            reqwest::header::ACCEPT,
            "application/vnd.heroku+json; version=3",
        )
        .send()
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(DeploymentsFetcherError::CommitIsNotFound)?
        .json::<HerokuSlugItem>()
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(DeploymentsFetcherError::CommitIsNotFound)?;

    Ok(slug)
}

async fn get_commit(
    github_owner_repo: ValidatedGitHubOwnerRepo,
    github_personal_token: ValidatedGitHubPersonalToken,
    sha: &str,
) -> Result<RepoCommit, DeploymentsFetcherError> {
    let octocrab = Octocrab::builder()
        .personal_token(github_personal_token.to_string())
        .build()
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(DeploymentsFetcherError::CreateAPIClientError)?;
    let commit: RepoCommit = octocrab
        .get(
            format!(
                "/repos/{owner}/{repo}/commits/{ref}",
                owner = github_owner_repo.get_owner(),
                repo = github_owner_repo.get_repo(),
                ref = &sha
            ),
            None::<&()>,
        )
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(DeploymentsFetcherError::CommitIsNotFound)?;

    Ok(commit)
}

async fn fetch_deployments(
    heroku_app_name: ValidatedHerokuAppName,
    heroku_auth_token: ValidatedHerokuAuthToken,
    _params: DeploymentsFetcherParams,
) -> Result<Vec<HerokuReleaseItem>, DeploymentsFetcherError> {
    let client = create_http_client();
    let url = format!(
        "https://api.heroku.com/apps/{app_name}/releases",
        app_name = heroku_app_name
    );
    let releases: Vec<HerokuReleaseItem> = client
        .get(url)
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}", token = heroku_auth_token),
        )
        .header(
            reqwest::header::ACCEPT,
            "application/vnd.heroku+json; version=3",
        )
        .header(reqwest::header::RANGE, "version ..; order=desc;")
        .send()
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(DeploymentsFetcherError::FetchError)?
        .json::<Vec<HerokuReleaseItem>>()
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .map_err(DeploymentsFetcherError::FetchError)?;

    let succeeded_releases = releases
        .into_iter()
        .filter(|release| release.status.to_uppercase() == "SUCCEEDED" && release.slug.is_some())
        .collect::<Vec<HerokuReleaseItem>>();

    Ok(succeeded_releases)
}

async fn attach_commit(
    heroku_app_name: ValidatedHerokuAppName,
    heroku_auth_token: ValidatedHerokuAuthToken,
    github_personal_token: ValidatedGitHubPersonalToken,
    github_owner_repo: ValidatedGitHubOwnerRepo,
    release: HerokuReleaseItem,
) -> Result<HerokuReleaseOrRepositoryInfo, DeploymentsFetcherError> {
    let slug_id =
        release
            .slug
            .clone()
            .map(|x| x.id)
            .ok_or(DeploymentsFetcherError::InvalidResponse(
                "slug is None".to_string(),
            ))?;
    let slug = get_slug(heroku_app_name, heroku_auth_token.clone(), &slug_id).await?;
    let commit = get_commit(github_owner_repo, github_personal_token, &slug.commit).await?;

    Ok(HerokuReleaseOrRepositoryInfo::HerokuRelease(
        HerokuRelease { release, commit },
    ))
}

fn convert_to_items(
    deployment_nodes: NonEmptyVec<HerokuReleaseOrRepositoryInfo>,
) -> Result<Vec<DeploymentLog>, DeploymentsFetcherError> {
    let mut sorted: NonEmptyVec<HerokuReleaseOrRepositoryInfo> = deployment_nodes;
    sorted.sort_by_key(|a| match a {
        HerokuReleaseOrRepositoryInfo::HerokuRelease(release) => release.release.created_at,
        HerokuReleaseOrRepositoryInfo::RepositoryInfo(info) => info.created_at,
    });
    log::debug!(
        "heroku releases: {:#?}",
        sorted
            .clone()
            .get_all()
            .iter()
            .map(|x| match x {
                HerokuReleaseOrRepositoryInfo::HerokuRelease(release) => format!(
                    "release: v{:?}, sha:{:?}, message:{:?}, created_at:{:?}",
                    release.release.version,
                    release.commit.sha,
                    release.commit.commit.message,
                    release.release.created_at
                ),
                HerokuReleaseOrRepositoryInfo::RepositoryInfo(info) =>
                    format!("repository {:?}", info.created_at),
            })
            .collect::<Vec<_>>()
    );
    let (first_item, rest) = sorted.get();

    // TODO: 無理やりすぎる
    let rest = rest
        .iter()
        .flat_map(|x| match x {
            HerokuReleaseOrRepositoryInfo::HerokuRelease(release) => Some(release.clone()),
            HerokuReleaseOrRepositoryInfo::RepositoryInfo(_info) => None,
        })
        .collect::<Vec<HerokuRelease>>();

    let first_commit: BaseCommitShaOrRepositoryInfo = match first_item {
        HerokuReleaseOrRepositoryInfo::HerokuRelease(item) => {
            BaseCommitShaOrRepositoryInfo::BaseCommitSha(item.commit.sha)
        }
        HerokuReleaseOrRepositoryInfo::RepositoryInfo(info) => {
            BaseCommitShaOrRepositoryInfo::RepositoryCreatedAt(info.created_at)
        }
    };

    let deployment_items = rest
        .iter()
        .scan(
            first_commit,
            |previous: &mut BaseCommitShaOrRepositoryInfo, release: &HerokuRelease| {
                let author_date = release.clone().commit.commit.author.and_then(|x| x.date);
                let author_login = release.clone().commit.author.map(|x| x.login);
                if author_date.is_none() || author_login.is_none() {
                    return None;
                }
                let commit_item = Commit {
                    sha: release.clone().commit.sha,
                    message: release.clone().commit.commit.message,
                    resource_path: release.clone().commit.html_url,
                    committed_at: release
                        .clone()
                        .commit
                        .commit
                        .author
                        .map(|x| x.date.unwrap())
                        .unwrap(),
                    creator_login: release.clone().commit.author.map(|x| x.login).unwrap(),
                };
                let deployment_item = DeploymentLog {
                    info: DeploymentInfo::HerokuRelease {
                        id: release.clone().release.id,
                        version: release.clone().release.version,
                    },
                    head_commit: commit_item,
                    base: previous.clone(),
                    creator_login: release.clone().commit.author.map(|x| x.login).unwrap(),
                    deployed_at: release.release.created_at,
                };
                *previous =
                    BaseCommitShaOrRepositoryInfo::BaseCommitSha(release.clone().commit.sha);
                Some(deployment_item)
            },
        )
        .collect::<Vec<DeploymentLog>>();

    Ok(deployment_items)
}

pub struct DeploymentsFetcherWithHerokuRelease {
    pub heroku_app_name: ValidatedHerokuAppName,
    pub heroku_auth_token: ValidatedHerokuAuthToken,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
}
#[async_trait]
impl DeploymentsFetcher for DeploymentsFetcherWithHerokuRelease {
    async fn fetch(
        &self,
        params: DeploymentsFetcherParams,
    ) -> Result<Vec<DeploymentLog>, DeploymentsFetcherError> {
        let succeeded_releases = fetch_deployments(
            self.heroku_app_name.clone(),
            self.heroku_auth_token.clone(),
            params,
        )
        .await?;
        let mut deployments = try_join_all(succeeded_releases.iter().map(|release| {
            attach_commit(
                self.heroku_app_name.clone(),
                self.heroku_auth_token.clone(),
                self.github_personal_token.clone(),
                self.github_owner_repo.clone(),
                release.clone(),
            )
        }))
        .await?;
        let repo_created_at = get_created_at(&self.github_personal_token, &self.github_owner_repo)
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .map_err(DeploymentsFetcherError::GetRepositoryCreatedAtError)?;
        log::debug!("repo_created_at: {:#?}", repo_created_at);
        deployments.push(HerokuReleaseOrRepositoryInfo::RepositoryInfo(
            GitHubRepositoryInfo {
                created_at: repo_created_at,
            },
        ));
        let non_empty_nodes = NonEmptyVec::new(deployments)
            .map_err(|e| anyhow::anyhow!(e))
            .map_err(DeploymentsFetcherError::DeploymentsFetcherResultIsEmptyList)?;

        let deployment_items = convert_to_items(non_empty_nodes)?;

        Ok(deployment_items)
    }
}
