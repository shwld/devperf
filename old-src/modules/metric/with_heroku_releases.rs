use chrono::{DateTime, Utc};
use crate::modules::types::deployment_metric::{DeploymentMetrics};
use crate::modules::heroku;

pub async fn calculate_metrics(owner: &str, repo: &str, since: DateTime<Utc>, until: DateTime<Utc>, developers: u64, working_days_per_week: f32, app_name: &str) -> Result<DeploymentMetrics, octocrab::Error> {
    let deployments = heroku::release::list(app_name, owner, repo).await.expect("Could not get deployments");

    let (ranged_deployments, first_committed_at) = super::metrics::filter_date_range(owner, repo, deployments, since, until).await;
    let metrics = super::deployment_lead_times::calculate(owner, repo, ranged_deployments, first_committed_at).await.expect("Could not calculate metrics");
    let deployment_metric = super::metrics::calculate(metrics, since, until, developers, working_days_per_week).await.expect("Could not calculate metrics");

    Ok(deployment_metric)
}
