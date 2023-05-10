use chrono::{DateTime, Utc};
use crate::modules::{types::deployment_metric::{DeploymentMetricItem, DeploymentCommitItem, DeploymentItem}, github::compare};

pub async fn calculate(owner: &str, repo: &str, deployment_nodes: Vec<DeploymentItem>, first_committed_at: DateTime<Utc>) -> Result<Vec<DeploymentMetricItem>, octocrab::Error> {
    let mut sorted = deployment_nodes.clone();
    sorted.sort_by_key(|a| a.deployed_at);

    let mut previous_commit_sha: String = "".to_string();
    let mut previous_commit_message: String = "".to_string();
    let mut previous_commit_resource_path: String = "".to_string();
    let mut previous_commit_committed_at = first_committed_at;
    let mut previous_commit_creator_login: String = "".to_string();
    let mut deployment_nodes: Vec<DeploymentMetricItem> = Vec::new();
    for deployment in sorted {
        let first_committed_at = if previous_commit_sha.is_empty() {
            first_committed_at
        } else {
            // FIXME: depending on module::github::compare::get_first_commit_committer_date() is not ideal
            let first_committed_at_from_compare = compare::get_first_commit_committer_date(owner, repo, &previous_commit_sha, &deployment.head_commit_sha).await.expect("Could not get first commit committer date");
            first_committed_at_from_compare
        };
        let lead_time_for_changes_seconds = (deployment.deployed_at - first_committed_at).num_seconds();
        let deployment_metric = DeploymentMetricItem {
            id: deployment.id,
            head_commit: DeploymentCommitItem {
                sha: deployment.head_commit_sha.clone(),
                message: deployment.head_commit_message.clone(),
                resource_path: deployment.head_commit_resource_path.clone(),
                committed_at: deployment.head_committed_at,
                creator_login: deployment.creator_login.clone(),
            },
            first_commit: DeploymentCommitItem {
                sha: previous_commit_sha.clone(),
                message: previous_commit_message.clone(),
                resource_path: previous_commit_resource_path.clone(),
                committed_at: previous_commit_committed_at,
                creator_login: previous_commit_creator_login.clone(),
            },
            deployed_at: deployment.deployed_at,
            lead_time_for_changes_seconds: lead_time_for_changes_seconds,
        };
        deployment_nodes.push(deployment_metric);
        previous_commit_sha = deployment.head_commit_sha.clone();
        previous_commit_message = deployment.head_commit_message.clone();
        previous_commit_resource_path = deployment.head_commit_resource_path.clone();
        previous_commit_committed_at = deployment.head_committed_at.clone();
        previous_commit_creator_login = deployment.creator_login.clone();
    }

    Ok(deployment_nodes)
}
