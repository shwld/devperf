#[cfg(test)]
mod tests {
    mod retrieve_four_keys_workflow_tests {
        use crate::{
            common_types::date_time_range::DateTimeRange,
            dependencies::{
                deployments_fetcher::mock::DeploymentsFetcherWithMock,
                first_commit_getter::mock::FirstCommitGetterWithMock,
            },
            metrics_retrieving::retrieve_four_keys::{
                RetrieveFourKeys, RetrieveFourKeysExecutionContext,
                RetrieveFourKeysExecutionContextProject, RetrieveFourKeysWorkflow,
            },
            shared::datetime_utc::parse,
        };

        #[tokio::test]
        async fn success() {
            let deployments_fetcher = DeploymentsFetcherWithMock {};
            let first_commit_getter = FirstCommitGetterWithMock {};
            let workflow = RetrieveFourKeysWorkflow {
                deployments_fetcher,
                first_commit_getter,
            };
            let since = parse("2023-01-01 00:00:00").expect("Could not parse since");
            let until = parse("2023-04-01 00:00:00").expect("Could not parse since");
            let timeframe = DateTimeRange::new(since, until).expect("Could not create timeframe");
            let context = RetrieveFourKeysExecutionContext {
                timeframe,
                project: RetrieveFourKeysExecutionContextProject {
                    name: "project".to_string(),
                    developer_count: 2,
                    working_days_per_week: 2.5,
                },
            };
            let result = workflow.retrieve_four_keys(context).await;
            assert!(result.is_ok())
        }
    }
}
