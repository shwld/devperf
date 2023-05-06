use crate::{dependencies::read_project_config::interface::{ReadProjectConfig, ProjectConfig}};

use super::{retrieve_four_keys__schema::{RetrieveFourKeysExecutionContext, RetrieveFourKeysEvent, RetrieveFourKeysEventError, DeploymentCommitItem}};

// ---------------------------
// Fetch deployments step
// ---------------------------

fn fetch_deployments(project_config: ProjectConfig) {
    unimplemented!()
}

// ---------------------------
// Calculation step
// ---------------------------


// ---------------------------
// overall workflow
// ---------------------------
pub async fn perform<TReadProjectConfig: ReadProjectConfig>(read_config: TReadProjectConfig, context: RetrieveFourKeysExecutionContext) -> Result<RetrieveFourKeysEvent, RetrieveFourKeysEventError> {
    let project_config = read_config.perform(context.project_name).await.map_err(RetrieveFourKeysEventError::ReadProjectConfigError)?;
    let deployments = fetch_deployments(project_config);
    unimplemented!()
}
