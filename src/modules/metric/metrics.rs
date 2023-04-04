use chrono::{DateTime, NaiveTime, Utc};
use crate::modules::types::deployment_metric::{DeploymentMetrics, DeploymentMetric, DeploymentMetricItem, DeploymentMetricSummary, DeploymentMetricLeadTimeForChanges, DeploymentItem};
use super::calc::median;
use crate::modules::github;

pub async fn calculate(metrics: Vec<DeploymentMetricItem>, since: DateTime<Utc>, until: DateTime<Utc>, developers: u64, working_days_per_week: f32) -> Result<DeploymentMetrics, octocrab::Error> {
    // 日付ごとにまとめて日付ごとの件数を取得する
    let mut deployment_frequencies_by_date: Vec<DeploymentMetricSummary> = Vec::new();
    let time = NaiveTime::from_hms_opt(0, 0, 0).expect("Could not create NaiveTime");
    // let mut date_counts = vec![];
    let mut total_deployments: u64 = 0;
    let mut current_date = since;
    let mut current_summary: Option<DeploymentMetricSummary> = None;
    let mut current_metrics: Vec<DeploymentMetricItem> = Vec::new();
    let mut current_date_count = 0;
    let mut durations: Vec<i64> = Vec::new();
    for metric in metrics {
        let target_time = metric.deployed_at.date_naive().and_time(time).and_local_timezone(Utc).unwrap();
        if target_time != current_date {
            current_date_count = 0;
            current_date = target_time;
            current_metrics = Vec::new();
            if current_summary.is_some() {
                deployment_frequencies_by_date.push(current_summary.expect("Could not get current_summary"));
            }
        }
        current_date_count += 1;
        total_deployments += 1;
        durations.push(metric.deployed_at.signed_duration_since(metric.first_commit.committed_at).num_seconds());
        current_metrics.push(metric);
        current_summary = Some(DeploymentMetricSummary {
            deploys: current_date_count,
            date: target_time.date_naive(),
            items: current_metrics.clone(),
        });
    }
    if current_summary.is_some() {
        deployment_frequencies_by_date.push(current_summary.expect("Could not get current_summary"));
    }

    // info
    let diff = until.signed_duration_since(since);
    let days = diff.num_days();
    let deployment_frequency_per_day = total_deployments as f32 / (days as f32 * (working_days_per_week / 7.0));
    let median_duration = median(durations);
    let hours = (median_duration / 3600.0) as i64;
    let minutes = ((median_duration.round() as i64 % 3600) / 60) as i64;
    let seconds = ((median_duration.round() as i64) - (hours * 3600) - (minutes * 60)) as i64;
    let lead_time = DeploymentMetricLeadTimeForChanges {
        hours: hours,
        minutes: minutes,
        seconds: seconds,
        total_seconds: median_duration,
    };
    let metrics = DeploymentMetric {
        since: since,
        until: until,
        developers: developers,
        working_days_per_week: working_days_per_week,
        deploys: total_deployments,
        deployment_frequency_per_day: deployment_frequency_per_day,
        deploys_a_day_a_developer: deployment_frequency_per_day / developers as f32,
        lead_time_for_changes: lead_time,
        environment: "production".to_string(), // TODO: get
    };

    let deployment_frequency = DeploymentMetrics {
        metrics: metrics,
        deployments: deployment_frequencies_by_date,
    };

    Ok(deployment_frequency)
}

pub async fn filter_date_range(owner: &str, repo: &str, deployments: Vec<DeploymentItem>, since: DateTime<Utc>, until: DateTime<Utc>) -> (Vec<DeploymentItem>, DateTime<Utc>) {
    // 期間内のdeploymentsだけにする
    let mut before_since_deployments: Vec<DeploymentItem> = Vec::new();
    let mut ranged_deployments: Vec<DeploymentItem> = Vec::new();
    for deployment in deployments {
        if deployment.deployed_at >= since && deployment.deployed_at <= until {
            ranged_deployments.push(deployment);
        } else if deployment.deployed_at < since {
            before_since_deployments.push(deployment);
        }
    }
    let first_committed_at = if before_since_deployments.len() > 0 && ranged_deployments.len() > 0 {
        let base = &before_since_deployments.last().expect("Could not get last").commit_sha;
        let head = &ranged_deployments.first().expect("Could not get first").commit_sha;
        // FIXME: depending on module::github::compare::get_first_commit_committer_date() is not ideal
        github::compare::get_first_commit_committer_date(owner, repo, base, head).await.expect("Could not get first_committed_at")
    } else {
        since
    };

    (ranged_deployments, first_committed_at)
}
