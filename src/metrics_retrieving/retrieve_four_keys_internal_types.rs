use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::retrieve_four_keys::{FirstCommitOrRepositoryInfo, RetrieveFourKeysEventError};
use crate::dependencies::deployments_fetcher::interface::DeploymentItem;

// ---------------------------
// Fetch deployments step
// ---------------------------
pub(super) struct FetchDeploymentsParams {
    pub(super) since: DateTime<Utc>,
    pub(super) until: DateTime<Utc>,
}
#[async_trait]
pub(super) trait FetchDeploymentsStep {
    async fn fetch_deployments(
        self,
        params: FetchDeploymentsParams,
    ) -> Result<Vec<DeploymentItem>, RetrieveFourKeysEventError>;
}

