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
            shared::datetime_utc::parse,
            test::factories::{
                deployment_item::build_deployment_item,
                first_commit_or_repository_info::build_first_commit,
                repository_info::build_repository_info,
            },
        };

        #[test]
        fn when_first_operation_is_none_should_none() {
            let item = DeploymentLogWithFirstOperation {
                deployment: build_deployment_item(parse("2023-01-01").unwrap()),
                first_operation: None,
            };
            assert_eq!(calculate_lead_time_for_changes_seconds(item), None);
        }

        #[test]
        fn when_first_operation_is_first_commit_should_get_seconds() {
            let item = DeploymentLogWithFirstOperation {
                deployment: build_deployment_item(parse("2023-01-05").unwrap()),
                first_operation: Some(FirstCommitOrRepositoryInfo::FirstCommit(
                    build_first_commit(parse("2023-01-01").unwrap()),
                )),
            };
            assert_eq!(
                calculate_lead_time_for_changes_seconds(item),
                Some(4 * 24 * 60 * 60)
            );
        }

        #[test]
        fn when_first_operation_is_repo_info_should_get_seconds() {
            let item = DeploymentLogWithFirstOperation {
                deployment: build_deployment_item(parse("2023-01-05").unwrap()),
                first_operation: Some(FirstCommitOrRepositoryInfo::RepositoryInfo(
                    build_repository_info(parse("2023-01-01").unwrap()),
                )),
            };
            assert_eq!(
                calculate_lead_time_for_changes_seconds(item),
                Some(4 * 24 * 60 * 60)
            );
        }
    }
}
