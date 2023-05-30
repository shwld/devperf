#[cfg(test)]
mod tests {
    mod calculate_lead_time_for_changes_seconds_tests {
        use crate::{
            metrics_retrieving::{
                retrieve_four_keys::{
                    calculate_lead_time_for_changes_seconds, FirstCommitOrRepositoryInfo,
                },
                retrieve_four_keys_internal_types::DeploymentLogWithFirstOperation,
            },
            test::factories::{
                commit::build_commit, deployment_log::build_deployment_log,
                repository_info::build_repository_info,
            },
        };

        #[test]
        fn when_first_operation_is_none_should_none() {
            let item = DeploymentLogWithFirstOperation {
                deployment_log: build_deployment_log("2023-01-01 00:00:00"),
                first_operation: None,
            };
            assert_eq!(calculate_lead_time_for_changes_seconds(item), None);
        }

        #[test]
        fn when_first_operation_is_first_commit_should_get_seconds() {
            let item = DeploymentLogWithFirstOperation {
                deployment_log: build_deployment_log("2023-01-05 00:00:00"),
                first_operation: Some(FirstCommitOrRepositoryInfo::FirstCommit(build_commit(
                    "2023-01-01 00:00:00",
                ))),
            };
            assert_eq!(
                calculate_lead_time_for_changes_seconds(item),
                Some(4 * 24 * 60 * 60)
            );
        }

        #[test]
        fn when_first_operation_is_repo_info_should_get_seconds() {
            let item = DeploymentLogWithFirstOperation {
                deployment_log: build_deployment_log("2023-01-05 00:00:00"),
                first_operation: Some(FirstCommitOrRepositoryInfo::RepositoryInfo(
                    build_repository_info("2023-01-01 00:00:00"),
                )),
            };
            assert_eq!(
                calculate_lead_time_for_changes_seconds(item),
                Some(4 * 24 * 60 * 60)
            );
        }
    }
}
