use chrono::{DateTime, Utc};

use crate::metrics_retrieving::retrieve_four_keys::DeploymentCommitItem;

pub fn build_first_commit(committed_at: DateTime<Utc>) -> DeploymentCommitItem {
    DeploymentCommitItem {
        sha: "sha".to_string(),
        message: "message".to_string(),
        resource_path: "resource_path".to_string(),
        committed_at,
        creator_login: "creator_login".to_string(),
    }
}
