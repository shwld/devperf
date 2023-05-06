use crate::project_creating::public_schema::ProjectConfig;

type ProjectName = String;
#[derive(Debug, Clone)]
pub struct ReadConfigError(pub String);
pub type ReadConfig = fn (ProjectName) -> Result<ProjectConfig, ReadConfigError>;
