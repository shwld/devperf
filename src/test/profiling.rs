#[cfg(test)]
mod tests {
    use crate::{
        common_types::date_time_range::DateTimeRange,
        dependencies::{
            deployments_fetcher::mock::DeploymentsFetcherWithMock,
            two_commits_comparer::mock::TwoCommitsComparerWithMock,
        },
        metrics_retrieving::retrieve_four_keys::{
            RetrieveFourKeys, RetrieveFourKeysExecutionContext,
            RetrieveFourKeysExecutionContextProject, RetrieveFourKeysWorkflow,
        },
        shared::datetime_utc::parse,
        test::factories::{commit::build_commit, deployment_log::build_deployment_log},
    };

    #[global_allocator]
    static ALLOC: dhat::Alloc = dhat::Alloc;

    #[tokio::test]
    async fn test() {
        let _profiler = dhat::Profiler::builder().testing().build();

        let context = RetrieveFourKeysExecutionContext {
            timeframe: DateTimeRange::new(
                parse("2023-01-01 00:00:00").expect("Could not parse since"),
                parse("2023-03-31 00:00:00").expect("Could not parse since"),
            )
            .expect("Could not create timeframe"),
            project: RetrieveFourKeysExecutionContextProject {
                name: "project".to_string(),
                developer_count: 2,
                working_days_per_week: 2.5,
            },
        };
        let deployments_fetcher = DeploymentsFetcherWithMock {
            deployment_logs: vec![
                build_deployment_log("2023-04-01 10:00:00"),
                build_deployment_log("2023-03-29 10:00:00"), // March    5th week
                build_deployment_log("2023-03-28 17:30:00"), // March    5th week
                build_deployment_log("2023-03-27 15:00:00"), // March    5th week
                build_deployment_log("2023-03-22 10:00:00"), // March    4th week
                build_deployment_log("2023-03-21 10:00:00"), // March    3th week
                build_deployment_log("2023-03-14 10:00:00"), // March    3th week
                build_deployment_log("2023-03-08 10:00:00"), // March    2nd week
                build_deployment_log("2023-03-07 10:00:00"), // March    2nd week
                build_deployment_log("2023-03-01 10:00:00"), // February 5th week
                build_deployment_log("2023-02-28 10:00:00"), // February 5th week
                build_deployment_log("2023-02-22 10:00:00"), // February 4th week
                build_deployment_log("2023-02-21 10:00:00"), // February 4th week
                build_deployment_log("2023-02-21 10:00:00"), // February 4th week
                build_deployment_log("2023-02-21 10:00:00"), // February 4th week
                build_deployment_log("2023-02-15 10:00:00"), // February 3th week
                build_deployment_log("2023-02-14 10:00:00"), // February 3th week
                build_deployment_log("2023-02-08 10:00:00"), // February 2nd week
                build_deployment_log("2023-02-07 10:00:00"), // February 2nd week
                build_deployment_log("2023-02-06 10:00:00"), // February 2nd week
                build_deployment_log("2023-02-01 10:00:00"), // January  5th week
                build_deployment_log("2023-01-31 10:00:00"), // January  5th week
                build_deployment_log("2023-01-25 10:00:00"), // January  4th week
                build_deployment_log("2023-01-24 10:00:00"), // January  4th week
                build_deployment_log("2023-01-17 10:00:00"), // January  3th week
                build_deployment_log("2023-01-16 10:00:00"), // January  3th week
                build_deployment_log("2023-01-10 10:00:00"), // January  2nd week
                build_deployment_log("2023-01-09 10:00:00"), // January  2nd week
                build_deployment_log("2023-01-04 10:00:00"), // January  1st week
                build_deployment_log("2023-01-03 10:00:00"), // January  1st week
                build_deployment_log("2023-01-02 10:00:00"), // January  1st week
                build_deployment_log("2022-12-31 10:00:00"),
            ],
        };
        let two_commits_comparer = TwoCommitsComparerWithMock {
            commits: vec![build_commit("2023-01-02 10:00:00")],
        };
        let workflow = RetrieveFourKeysWorkflow {
            deployments_fetcher,
            two_commits_comparer,
        };
        workflow
            .retrieve_four_keys(context)
            .await
            .expect("Could not retrieve four keys");

        let stats = dhat::HeapStats::get();

        println!("stats: {:#?}", stats);
    }
}
