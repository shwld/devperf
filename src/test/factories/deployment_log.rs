// use chrono::Utc;

// use crate::{
//     dependencies::deployments_fetcher::interface::{
//         BaseCommitShaOrRepositoryInfo, CommitItem, DeploymentInfo, DeploymentLog,
//     },
//     shared::datetime_utc::parse,
// };

// pub fn build_deployment_log(deployed_at_str: &str) -> DeploymentLog {
//     let deployed_at = parse(deployed_at_str).expect("Could not parse deployed_at_str");
//     DeploymentLog {
//         info: DeploymentInfo::GithubDeployment {
//             id: "id".to_string(),
//         },
//         head_commit: CommitItem {
//             sha: "sha".to_string(),
//             message: "message".to_string(),
//             resource_path: "resource_path".to_string(),
//             committed_at: Utc::now(),
//             creator_login: "creator_login".to_string(),
//         },
//         base: BaseCommitShaOrRepositoryInfo::BaseCommitSha("base".to_string()),
//         creator_login: "creator_login".to_string(),
//         deployed_at,
//     }
// }
