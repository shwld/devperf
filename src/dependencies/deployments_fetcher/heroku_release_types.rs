use serde::{Deserialize, Serialize};

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
