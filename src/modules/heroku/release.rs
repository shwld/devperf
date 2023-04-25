use reqwest::Error;
use serde::{Serialize, Deserialize};

pub async fn list(app_name: &str) -> Result<(), Error> {
    let config = crate::modules::config::load_config().await;
    if config.heroku_token.is_none() {
        panic!("You must login first.");
    }
    let heroku_token = config.heroku_token.unwrap();
    let client = reqwest::Client::new();
    let url = format!("https://api.heroku.com/apps/{app_name}/releases", app_name = app_name);
    let resp = client.get(url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}", token = heroku_token))
        .header(reqwest::header::ACCEPT, "application/vnd.heroku+json; version=3")
        .header(reqwest::header::RANGE, "version ..; order=desc;")
        .send().await?
        .json::<Vec<HerokuReleaseItem>>().await?;
    println!("{:#?}", resp);
    Ok(())
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
