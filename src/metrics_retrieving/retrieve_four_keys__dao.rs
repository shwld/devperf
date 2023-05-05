use crate::project_creating::schema::ProjectConfig;

// Error types
#[derive(Debug, Clone)]
pub struct ReadConfigError(pub String);

type ProjectName = String;
pub type ReadConfig = fn (ProjectName) -> Result<ProjectConfig, ReadConfigError>;
