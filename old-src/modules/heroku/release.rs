use reqwest::Error;
use serde::{Serialize, Deserialize};
use crate::modules::{types::deployment_metric::{DeploymentItem}, github, http_client::create_http_client};

// FIXME: remove depends modules::github
pub async fn list(app_name: &str, owner: &str, repo: &str) -> Result<Vec<DeploymentItem>, Error> {
    let config = crate::modules::config::load_config().await;
    if config.heroku_token.is_none() {
        panic!("You must login first.");
    }
    let heroku_token = config.heroku_token.unwrap();
    let client = create_http_client();
    let url = format!("https://api.heroku.com/apps/{app_name}/releases", app_name = app_name);
    let releases: Vec<HerokuReleaseItem> = client.get(url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}", token = heroku_token))
        .header(reqwest::header::ACCEPT, "application/vnd.heroku+json; version=3")
        .header(reqwest::header::RANGE, "version ..; order=desc;")
        .send().await.expect("Could not get releases")
        .json::<Vec<HerokuReleaseItem>>().await?;

    let mut deployments: Vec<DeploymentItem> = Vec::new();
    let mut i = 0;
    for release in releases {
        i += 1;
        if i > 100 {
            break;}
        let status = release.status.to_uppercase();
        if status != "SUCCEEDED" || release.slug.is_none() {
            continue;
        }
        let slug_id = release.slug.unwrap().id;
        let slug = get_slug(app_name, &slug_id).await?;
        let commit = github::commit::get(owner, repo, &slug.commit).await.expect("commit not found");
        let commit_cloned = commit.clone();
        log::debug!("deployment adding slug: {:?}, {:?}", slug, commit_cloned.commit);
        let committed_at = commit.commit.author.and_then(|author| author.date).or_else(|| commit.commit.comitter.and_then(|comitter| comitter.date)).expect("commit date not found");
        let login = commit.author.map(|author| author.login).or_else(|| commit.committer.map(|committer| committer.login)).expect("commiter not found");
        let deployment = DeploymentItem {
            id: release.id,
            head_commit_sha: commit.sha,
            head_commit_message: commit.commit.message,
            head_commit_resource_path: commit.html_url,
            head_committed_at: committed_at,
            creator_login: login,
            deployed_at: release.created_at,
        };

        log::debug!("deployment added {:?}", deployment.head_commit_message);
        deployments.push(deployment);
    }
    log::debug!("{:#?}", deployments);

    Ok(deployments)
}

async fn get_slug(app_name: &str, slug_id: &str) -> Result<HerokuSlugItem, Error> {
    let config = crate::modules::config::load_config().await;
    if config.heroku_token.is_none() {
        panic!("You must login first.");
    }
    let heroku_token = config.heroku_token.unwrap();
    let client = create_http_client();
    let url = format!("https://api.heroku.com/apps/{app_name}/slugs/{slug_id}", app_name = app_name, slug_id = slug_id);
    let slug = client.get(url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}", token = heroku_token))
        .header(reqwest::header::ACCEPT, "application/vnd.heroku+json; version=3")
        .send().await.expect("Could not get slug")
        .json::<HerokuSlugItem>().await?;

    Ok(slug)
}

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
