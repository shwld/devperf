use crate::{metrics_retrieving::retrieve_four_keys::RepositoryInfo, shared::datetime_utc::parse};

pub fn build_repository_info(created_at_str: &str) -> RepositoryInfo {
    let created_at = parse(created_at_str).expect("Could not parse created_at_str");
    RepositoryInfo { created_at }
}
