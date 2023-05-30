// use crate::{
//     metrics_retrieving::retrieve_four_keys::DeploymentCommitItem, shared::datetime_utc::parse,
// };

// pub fn build_first_commit(committed_at_str: &str) -> DeploymentCommitItem {
//     let committed_at = parse(committed_at_str).expect("Could not parse committed_at_str");
//     DeploymentCommitItem {
//         sha: "sha".to_string(),
//         message: "message".to_string(),
//         resource_path: "resource_path".to_string(),
//         committed_at,
//         creator_login: "creator_login".to_string(),
//     }
// }
