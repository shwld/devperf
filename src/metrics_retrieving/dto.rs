use thiserror::Error;

use crate::{
    project_creating::dto::ProjectConfigDto,
    project_parameter_validating::{
        validate_developer_count::{self, ValidateDeveloperCountError},
        validate_working_days_per_week::{self, ValidateWorkingDaysPerWeekError},
    },
};

use super::retrieve_four_keys_public_types::RetrieveFourKeysExecutionContextProject;

#[derive(Error, Debug)]
pub enum RetrieveFourKeysExecutionContextDtoError {
    #[error("Developer count is invalid")]
    InvalidDeveloperCount(#[from] ValidateDeveloperCountError),
    #[error("Developer count is invalid")]
    InvalidWorkingDaysPerWeek(#[from] ValidateWorkingDaysPerWeekError),
}

struct RetrieveFourKeysExecutionContextDto;
impl RetrieveFourKeysExecutionContextDto {
    fn build_context(
        dto: ProjectConfigDto,
    ) -> Result<RetrieveFourKeysExecutionContextProject, RetrieveFourKeysExecutionContextDtoError>
    {
        let developer_count = validate_developer_count::perform(dto.developer_count.to_string())?;
        let working_days_per_week =
            validate_working_days_per_week::perform(dto.working_days_per_week.to_string())?;
        Ok(RetrieveFourKeysExecutionContextProject {
            name: dto.project_name,
            developer_count: developer_count.to_u32(),
            working_days_per_week: working_days_per_week.to_f32(),
        })
    }
}
