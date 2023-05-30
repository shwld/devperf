use async_trait::async_trait;

use crate::test::factories::deployment_log::build_deployment_log;

use super::interface::{
    DeploymentLog, DeploymentsFetcher, DeploymentsFetcherError, DeploymentsFetcherParams,
};

pub struct DeploymentsFetcherWithMock {}
#[async_trait]
impl DeploymentsFetcher for DeploymentsFetcherWithMock {
    async fn fetch(
        &self,
        _params: DeploymentsFetcherParams,
    ) -> Result<Vec<DeploymentLog>, DeploymentsFetcherError> {
        let deployment_items: Vec<DeploymentLog> = vec![
            build_deployment_log("2021-04-10 00:00:00"),
            build_deployment_log("2021-04-01 00:00:00"),
            build_deployment_log("2021-03-20 00:00:00"),
            build_deployment_log("2021-03-20 00:00:00"),
            build_deployment_log("2021-03-03 00:00:00"),
            build_deployment_log("2021-01-01 00:00:00"),
        ];

        Ok(deployment_items)
    }
}
