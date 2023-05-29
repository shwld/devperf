use chrono::{DateTime, Utc};

use crate::{ metrics_retrieving::retrieve_four_keys::RepositoryInfo};

pub fn build_repository_info(created_at: DateTime<Utc>) -> RepositoryInfo {
    RepositoryInfo { created_at }
}
