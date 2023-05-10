use crate::dependencies::write_new_config::interface::WriteNewConfig;

use super::dto::HerokuReleaseProjectCreatedDto;
use super::schema::*;

pub async fn perform<T: WriteNewConfig>(
    write_new_config: T,
    project: UncreatedHerokuReleaseProject,
) -> Result<CreateHerokuReleaseProjectEvent, CreateHerokuReleaseProjectError> {
    let project = HerokuReleaseProjectCreated {
        project_name: project.project_name,
        github_personal_token: project.github_personal_token,
        github_owner_repo: project.github_owner_repo,
        heroku_app_name: project.heroku_app_name,
        heroku_api_token: project.heroku_api_token,
        developer_count: project.developer_count,
        working_days_per_week: project.working_days_per_week,
    };
    let project_dto =
        HerokuReleaseProjectCreatedDto::from_heroku_release_project_created(project.clone());
    write_new_config
        .perform(project_dto)
        .await
        .map_err(CreateHerokuReleaseProjectError::WriteNewConfigError)?;
    Ok(project)
}
