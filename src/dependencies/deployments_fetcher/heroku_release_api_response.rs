use serde::{Deserialize, Serialize};

//------------------------
// Heroku Release API
//------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct HerokuReleaseItem {
    pub(super) addon_plan_names: Vec<String>,
    pub(super) app: HerokuReleaseApp,
    pub(super) created_at: chrono::DateTime<chrono::Utc>,
    pub(super) description: String,
    pub(super) status: String,
    pub(super) id: String,
    pub(super) slug: Option<HerokuReleaseSlug>,
    pub(super) updated_at: chrono::DateTime<chrono::Utc>,
    pub(super) user: HerokuReleaseUser,
    pub(super) version: u64,
    pub(super) current: bool,
    pub(super) output_stream_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct HerokuReleaseApp {
    pub(super) id: String,
    pub(super) name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct HerokuReleaseSlug {
    pub(super) id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct HerokuReleaseUser {
    pub(super) email: String,
    pub(super) id: String,
}

//------------------------
// Heroku Slug API
//------------------------
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct HerokuSlugItem {
    pub(super) blob: HerokuSlugBlobItem,
    pub(super) buildpack_provided_description: String,
    pub(super) checksum: String,
    pub(super) commit: String,
    pub(super) commit_description: String,
    pub(super) created_at: chrono::DateTime<chrono::Utc>,
    pub(super) id: String,
    pub(super) size: u64,
    pub(super) stack: HerokuSlugStackItem,
    pub(super) updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct HerokuSlugBlobItem {
    pub(super) method: String,
    pub(super) url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub(super) struct HerokuSlugStackItem {
    pub(super) id: String,
    pub(super) name: String,
}
