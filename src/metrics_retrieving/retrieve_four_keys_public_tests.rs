#[cfg(test)]
mod tests {
    mod retrieve_four_keys_workflow_tests {
        use crate::{
            common_types::date_time_range::DateTimeRange,
            dependencies::{
                deployments_fetcher::mock::DeploymentsFetcherWithMock,
                two_commits_comparer::mock::TwoCommitsComparerWithMock,
            },
            metrics_retrieving::retrieve_four_keys::{
                DeploymentFrequencyLabel, DeploymentFrequencyPerformanceSurvey2022,
                RetrieveFourKeys, RetrieveFourKeysEvent, RetrieveFourKeysExecutionContext,
                RetrieveFourKeysExecutionContextProject, RetrieveFourKeysWorkflow,
            },
            shared::datetime_utc::parse,
            test::factories::{commit::build_commit, deployment_log::build_deployment_log},
        };

        #[tokio::test]
        async fn deployment_performance_is_high() {
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
                    // total deploys    = 30
                    // all days         = 90
                    //
                    // weekly_deploys
                    //   January  1st week: 3
                    //   January  2nd week: 2
                    //   January  3th week: 2
                    //   January  4th week: 4
                    //   January  5th week: 2
                    //   February 2nd week: 3
                    //   February 3th week: 2
                    //   February 4th week: 2
                    //   February 5th week: 2
                    //   March    2nd week: 2
                    //   March    3th week: 2
                    //   March    4th week: 1
                    //   March    5th week: 3
                    //   [4,3,3,3,2,2,(2),2,2,2,2,2,1]
                    //   -> Median:         2.0
                    //
                    // deployed_weekly
                    //   January 1st week:  true(1)
                    //   January 2nd week:  true(1)
                    //   January 3th week:  true(1)
                    //   January 4th week:  true(1)
                    //   January 5th week:  true(1)
                    //   February 2nd week: true(1)
                    //   February 3th week: true(1)
                    //   February 4th week: true(1)
                    //   February 5th week: true(1)
                    //   March 2nd week:    true(1)
                    //   March 3th week:    true(1)
                    //   March 4th week:    true(1)
                    //   March 5th week:    true(1)
                    //   -> Median:         1.0
                    //
                    // deployed_monthly
                    //   January:           true(1)
                    //   February:          true(1)
                    //   March:             true(1)
                    //   -> Median:         1.0
                    //
                    // per day
                    //   28days / (90days * (2.5 / 7)) = 0.8711111111
                    // per day per a developer
                    //   0.8711111111 / 2 = 0.4355555555
                    //
                    // performance
                    //   2.0(weekly_deploys_median) > (3days(DORA defined) * (2.5working days / 5weekday)) -> Daily, High
                ],
            };
            let two_commits_comparer = TwoCommitsComparerWithMock {
                commits: vec![build_commit("2023-01-02 10:00:00")],
            };
            let workflow = RetrieveFourKeysWorkflow {
                deployments_fetcher,
                two_commits_comparer,
            };
            let result = workflow.retrieve_four_keys(context).await;
            assert!(result.is_ok());

            for item in result.unwrap() {
                match item {
                    RetrieveFourKeysEvent::RetrieveFourKeys(result) => {
                        let all_days = result.deployments.len();
                        let frequency = result.performance.deployment_frequency.value;
                        let label = result.performance.deployment_frequency.label;
                        let performance = result.performance.deployment_frequency.performance;
                        assert_eq!(all_days, 90);
                        assert_eq!(frequency.total_deployments, 30);
                        assert_eq!(frequency.weekly_deployment_count_median, 2.0);
                        assert_eq!(frequency.month_deployed_median, 1.0);
                        assert_eq!(frequency.week_deployed_median, 1.0);
                        assert_eq!(frequency.deployment_frequency_per_day, 0.871_111_15);
                        assert_eq!(frequency.deploys_per_a_day_per_a_developer, 0.435_555_58);
                        assert_eq!(label, DeploymentFrequencyLabel::Daily);
                        assert_eq!(performance, DeploymentFrequencyPerformanceSurvey2022::High);
                    }
                }
            }
        }

        #[tokio::test]
        async fn deployment_performance_is_medium() {
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
                    build_deployment_log("2023-03-22 10:00:00"), // March    4th week
                    build_deployment_log("2023-03-14 10:00:00"), // March    3th week
                    build_deployment_log("2023-02-28 10:00:00"), // February 5th week
                    build_deployment_log("2023-02-22 10:00:00"), // February 4th week
                    build_deployment_log("2023-02-08 10:00:00"), // February 2nd week
                    build_deployment_log("2023-01-31 10:00:00"), // January  5th week
                    build_deployment_log("2023-01-25 10:00:00"), // January  4th week
                    build_deployment_log("2023-01-10 10:00:00"), // January  2nd week
                    build_deployment_log("2023-01-02 10:00:00"), // January  1st week
                    build_deployment_log("2022-12-31 10:00:00"),
                    // total deploys    = 11
                    // all days         = 90
                    //
                    // weekly_deploys
                    //   January  1st week: 1
                    //   January  2nd week: 1
                    //   January  3th week: 0
                    //   January  4th week: 1
                    //   January  5th week: 1
                    //   February 2nd week: 1
                    //   February 3th week: 0
                    //   February 4th week: 1
                    //   February 5th week: 1
                    //   March    2nd week: 0
                    //   March    3th week: 1
                    //   March    4th week: 1
                    //   March    5th week: 1
                    //   [0,0,0,1,1,1,(1),1,1,1,1,1,1]
                    //   -> Median:         1.0
                    //
                    // deployed_weekly
                    //   January 1st week:  true(1)
                    //   January 2nd week:  true(1)
                    //   January 3th week:  false(0)
                    //   January 4th week:  true(1)
                    //   January 5th week:  true(1)
                    //   February 2nd week: true(1)
                    //   February 3th week: false(0)
                    //   February 4th week: true(1)
                    //   February 5th week: true(1)
                    //   March 2nd week:    false(0)
                    //   March 3th week:    true(1)
                    //   March 4th week:    true(1)
                    //   March 5th week:    true(1)
                    //   [0,0,0,1,1,1,(1),1,1,1,1,1,1]
                    //   -> Median:         1.0
                    //
                    // deployed_monthly
                    //   January:           true(1)
                    //   February:          true(1)
                    //   March:             true(1)
                    //   -> Median:         1.0
                    //
                    // per day
                    //   11days / (90days * (2.5 / 7)) = 0.3422222222
                    // per day per a developer
                    //   0.3422222222 / 2 = 0.1711111111
                    //
                    // performance
                    //   1.0(weekly_deploys_median) > (3days(DORA defined) * (2.5working days / 5weekday)) -> Not enough
                    //   1.0(deployed_weekly) >= 1.0(DORA defined) -> Medium, Weekly
                ],
            };
            let two_commits_comparer = TwoCommitsComparerWithMock {
                commits: vec![build_commit("2023-01-02 10:00:00")],
            };
            let workflow = RetrieveFourKeysWorkflow {
                deployments_fetcher,
                two_commits_comparer,
            };
            let result = workflow.retrieve_four_keys(context).await;
            assert!(result.is_ok());

            for item in result.unwrap() {
                match item {
                    RetrieveFourKeysEvent::RetrieveFourKeys(result) => {
                        let all_days = result.deployments.len();
                        let frequency = result.performance.deployment_frequency.value;
                        let label = result.performance.deployment_frequency.label;
                        let performance = result.performance.deployment_frequency.performance;
                        assert_eq!(all_days, 90);
                        assert_eq!(frequency.total_deployments, 11);
                        assert_eq!(frequency.weekly_deployment_count_median, 1.0);
                        assert_eq!(frequency.month_deployed_median, 1.0);
                        assert_eq!(frequency.week_deployed_median, 1.0);
                        assert_eq!(frequency.deployment_frequency_per_day, 0.342_222_2);
                        assert_eq!(frequency.deploys_per_a_day_per_a_developer, 0.171_111_1);
                        assert_eq!(label, DeploymentFrequencyLabel::Weekly);
                        assert_eq!(
                            performance,
                            DeploymentFrequencyPerformanceSurvey2022::Medium
                        );
                    }
                }
            }
        }

        #[tokio::test]
        async fn deployment_performance_is_low() {
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
                    build_deployment_log("2023-01-31 10:00:00"), // January  5th week
                    build_deployment_log("2023-01-02 10:00:00"), // January  1st week
                    build_deployment_log("2022-12-31 10:00:00"),
                    // total deploys    = 3
                    // all days         = 90
                    //
                    // weekly_deploys
                    //   January  1st week: 1
                    //   January  2nd week: 0
                    //   January  3th week: 0
                    //   January  4th week: 0
                    //   January  5th week: 1
                    //   February 2nd week: 0
                    //   February 3th week: 0
                    //   February 4th week: 0
                    //   February 5th week: 0
                    //   March    2nd week: 0
                    //   March    3th week: 0
                    //   March    4th week: 0
                    //   March    5th week: 1
                    //   [0,0,0,0,0,0,(0),0,0,0,1,1,1]
                    //   -> Median:         0.0
                    //
                    // deployed_weekly
                    //   January 1st week:  true(1)
                    //   January 2nd week:  false(0)
                    //   January 3th week:  false(0)
                    //   January 4th week:  false(0)
                    //   January 5th week:  true(1)
                    //   February 2nd week: false(0)
                    //   February 3th week: false(0)
                    //   February 4th week: false(0)
                    //   February 5th week: false(0)
                    //   March 2nd week:    false(0)
                    //   March 3th week:    false(0)
                    //   March 4th week:    false(0)
                    //   March 5th week:    true(1)
                    //   [0,0,0,0,0,0,(0),0,0,0,1,1,1]
                    //   -> Median:         0.0
                    //
                    // deployed_monthly
                    //   January:           true(1)
                    //   February:          true(0)
                    //   March:             true(1)
                    //   [0,(1),1]
                    //   -> Median:         1.0
                    //
                    // per day
                    //   3days / (90days * (2.5 / 7)) = 0.0933333333
                    // per day per a developer
                    //   0.0933333333 / 2 = 0.0466666667
                    //
                    // performance
                    //   0.0(weekly_deploys_median) > (3days(DORA defined) * (2.5working days / 5weekday)) -> Not enough
                    //   0.0(deployed_weekly) >= 1.0(DORA defined) -> Not enough
                    //   1.0(deployed_monthly) >= 1.0(DORA defined) -> Low, Monthly
                ],
            };
            let two_commits_comparer = TwoCommitsComparerWithMock {
                commits: vec![build_commit("2023-01-02 10:00:00")],
            };
            let workflow = RetrieveFourKeysWorkflow {
                deployments_fetcher,
                two_commits_comparer,
            };
            let result = workflow.retrieve_four_keys(context).await;
            assert!(result.is_ok());

            for item in result.unwrap() {
                match item {
                    RetrieveFourKeysEvent::RetrieveFourKeys(result) => {
                        let all_days = result.deployments.len();
                        let frequency = result.performance.deployment_frequency.value;
                        let label = result.performance.deployment_frequency.label;
                        let performance = result.performance.deployment_frequency.performance;
                        assert_eq!(all_days, 90);
                        assert_eq!(frequency.total_deployments, 3);
                        assert_eq!(frequency.weekly_deployment_count_median, 0.0);
                        assert_eq!(frequency.month_deployed_median, 1.0);
                        assert_eq!(frequency.week_deployed_median, 0.0);
                        assert_eq!(frequency.deployment_frequency_per_day, 0.093_333_334);
                        assert_eq!(frequency.deploys_per_a_day_per_a_developer, 0.046_666_667);
                        assert_eq!(label, DeploymentFrequencyLabel::Monthly);
                        assert_eq!(performance, DeploymentFrequencyPerformanceSurvey2022::Low);
                    }
                }
            }
        }

        #[tokio::test]
        async fn deployment_performance_is_low_yearly() {
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
                    build_deployment_log("2023-01-31 10:00:00"), // January  5th week
                    build_deployment_log("2023-01-02 10:00:00"), // January  1st week
                    build_deployment_log("2022-12-31 10:00:00"),
                    // total deploys    = 2
                    // all days         = 90
                    //
                    // weekly_deploys
                    //   January  1st week: 1
                    //   January  2nd week: 0
                    //   January  3th week: 0
                    //   January  4th week: 0
                    //   January  5th week: 1
                    //   February 2nd week: 0
                    //   February 3th week: 0
                    //   February 4th week: 0
                    //   February 5th week: 0
                    //   March    2nd week: 0
                    //   March    3th week: 0
                    //   March    4th week: 0
                    //   March    5th week: 0
                    //   [0,0,0,0,0,0,(0),0,0,0,0,1,1]
                    //   -> Median:         0.0
                    //
                    // deployed_weekly
                    //   January 1st week:  true(1)
                    //   January 2nd week:  false(0)
                    //   January 3th week:  false(0)
                    //   January 4th week:  false(0)
                    //   January 5th week:  true(1)
                    //   February 2nd week: false(0)
                    //   February 3th week: false(0)
                    //   February 4th week: false(0)
                    //   February 5th week: false(0)
                    //   March 2nd week:    false(0)
                    //   March 3th week:    false(0)
                    //   March 4th week:    false(0)
                    //   March 5th week:    false(0)
                    //   [0,0,0,0,0,0,(0),0,0,0,0,1,1]
                    //   -> Median:         0.0
                    //
                    // deployed_monthly
                    //   January:           true(1)
                    //   February:          true(0)
                    //   March:             true(0)
                    //   [0,(0),1]
                    //   -> Median:         0.0
                    //
                    // per day
                    //   2days / (90days * (2.5 / 7)) = 0.0622222222
                    // per day per a developer
                    //   0.0622222222 / 2 = 0.0311111111
                    //
                    // performance
                    //   0.0(weekly_deploys_median) > (3days(DORA defined) * (2.5working days / 5weekday)) -> Not enough
                    //   0.0(deployed_weekly) >= 1.0(DORA defined) -> Not enough
                    //   0.0(deployed_monthly) >= 1.0(DORA defined) -> Not enough
                    //   -> Low, Yearly
                ],
            };
            let two_commits_comparer = TwoCommitsComparerWithMock {
                commits: vec![build_commit("2023-01-02 10:00:00")],
            };
            let workflow = RetrieveFourKeysWorkflow {
                deployments_fetcher,
                two_commits_comparer,
            };
            let result = workflow.retrieve_four_keys(context).await;
            assert!(result.is_ok());

            for item in result.unwrap() {
                match item {
                    RetrieveFourKeysEvent::RetrieveFourKeys(result) => {
                        let all_days = result.deployments.len();
                        let frequency = result.performance.deployment_frequency.value;
                        let label = result.performance.deployment_frequency.label;
                        let performance = result.performance.deployment_frequency.performance;
                        assert_eq!(all_days, 90);
                        assert_eq!(frequency.total_deployments, 2);
                        assert_eq!(frequency.weekly_deployment_count_median, 0.0);
                        assert_eq!(frequency.month_deployed_median, 0.0);
                        assert_eq!(frequency.week_deployed_median, 0.0);
                        assert_eq!(frequency.deployment_frequency_per_day, 0.062_222_224);
                        assert_eq!(frequency.deploys_per_a_day_per_a_developer, 0.031_111_112);
                        assert_eq!(label, DeploymentFrequencyLabel::Yearly);
                        assert_eq!(performance, DeploymentFrequencyPerformanceSurvey2022::Low);
                    }
                }
            }
        }
    }
}
