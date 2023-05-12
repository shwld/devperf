use thiserror::Error;

use crate::{
    common_types::{
        developer_count::{ValidateDeveloperCountError, ValidatedDeveloperCount},
        working_days_per_week::{ValidateWorkingDaysPerWeekError, ValidatedWorkingDaysPerWeek},
    },
    project_creating::dto::ProjectConfigDto,
};

use super::retrieve_four_keys_public_types::RetrieveFourKeysExecutionContextProject;

#[derive(Error, Debug)]
pub enum RetrieveFourKeysExecutionContextDtoError {
    #[error("Developer count is invalid")]
    InvalidDeveloperCount(#[from] ValidateDeveloperCountError),
    #[error("Developer count is invalid")]
    InvalidWorkingDaysPerWeek(#[from] ValidateWorkingDaysPerWeekError),
}

pub struct RetrieveFourKeysExecutionContextDto;
impl RetrieveFourKeysExecutionContextDto {
    pub fn build_context(
        dto: ProjectConfigDto,
    ) -> Result<RetrieveFourKeysExecutionContextProject, RetrieveFourKeysExecutionContextDtoError>
    {
        let developer_count = ValidatedDeveloperCount::new(dto.developer_count.to_string())?;
        let working_days_per_week =
            ValidatedWorkingDaysPerWeek::new(dto.working_days_per_week.to_string())?;
        Ok(RetrieveFourKeysExecutionContextProject {
            name: dto.project_name,
            developer_count: developer_count.to_u32(),
            working_days_per_week: working_days_per_week.to_f32(),
        })
    }
}
