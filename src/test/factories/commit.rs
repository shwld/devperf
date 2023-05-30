use crate::{common_types::commit::Commit, shared::datetime_utc::parse};

pub fn build_commit(committed_at_str: &str) -> Commit {
    let committed_at = parse(committed_at_str).expect("Could not parse committed_at_str");
    Commit {
        sha: "sha".to_string(),
        message: "message".to_string(),
        resource_path: "resource_path".to_string(),
        committed_at,
        creator_login: "creator_login".to_string(),
    }
}
