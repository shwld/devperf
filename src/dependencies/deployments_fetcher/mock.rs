use async_trait::async_trait;

use super::interface::{
    DeploymentLog, DeploymentsFetcher, DeploymentsFetcherError, DeploymentsFetcherParams,
};

pub struct DeploymentsFetcherWithMock {
    pub deployment_logs: Vec<DeploymentLog>,
}
#[async_trait]
impl DeploymentsFetcher for DeploymentsFetcherWithMock {
    async fn fetch(
        &self,
        _params: DeploymentsFetcherParams,
    ) -> Result<Vec<DeploymentLog>, DeploymentsFetcherError> {
        Ok(self.deployment_logs.clone())
    }
}
