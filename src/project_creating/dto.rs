use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::project_parameter_validating::{
    validate_developer_count::{self, ValidateDeveloperCountError},
    validate_github_owner_repo::{self, ValidateGitHubOwnerRepoError},
    validate_github_personal_token::{self, ValidateGitHubPersonalTokenError},
    validate_heroku_app_name::{self, ValidateHerokuAppNameError},
    validate_heroku_auth_token::{self, ValidateHerokuAuthTokenError},
    validate_working_days_per_week::{self, ValidateWorkingDaysPerWeekError},
};

use super::create_project_public_types::{
    GitHubDeploymentProjectCreated, HerokuReleaseProjectCreated, ProjectCreated,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfigDto {
    pub project_name: String,
    pub github_personal_token: String,
    pub github_owner: String,
    pub github_repo: String,
    pub heroku_app_name: Option<String>,
    pub heroku_auth_token: Option<String>,
    pub developer_count: u32,
    pub working_days_per_week: f32,
    pub deployment_source: String,
}

#[derive(Error, Debug)]
pub enum CreateProjectDtoError {
    #[error("GitHub personal token is invalid")]
    GitHubPersonalToken(#[from] ValidateGitHubPersonalTokenError),
    #[error("GitHub owner/repo is invalid")]
    GitHubOwnerRepo(#[from] ValidateGitHubOwnerRepoError),
    #[error("GitHub developer count is invalid")]
    DeveloperCount(#[from] ValidateDeveloperCountError),
    #[error("GitHub working days per week is invalid")]
    WorkingDaysPerWeek(#[from] ValidateWorkingDaysPerWeekError),
    #[error("GitHub personal token is invalid")]
    HerokuAuthToken(#[from] ValidateHerokuAuthTokenError),
    #[error("GitHub owner/repo is invalid")]
    HerokuAppName(#[from] ValidateHerokuAppNameError),
    #[error("Data source type is invalid")]
    InvalidDataSource(String),
}

fn to_github_deployment_project_created(
    dto: &ProjectConfigDto,
) -> Result<GitHubDeploymentProjectCreated, CreateProjectDtoError> {
    let github_personal_token =
        validate_github_personal_token::perform(Some(dto.github_personal_token.clone()))?;
    let github_owner_repo =
        validate_github_owner_repo::perform(format!("{}/{}", dto.github_owner, dto.github_repo))?;
    let developer_count = validate_developer_count::perform(dto.developer_count.to_string())?;
    let working_days_per_week =
        validate_working_days_per_week::perform(dto.working_days_per_week.to_string())?;
    Ok(GitHubDeploymentProjectCreated {
        project_name: dto.project_name.to_string(),
        github_personal_token,
        github_owner_repo,
        developer_count,
        working_days_per_week,
    })
}

fn from_github_deployment_project_created(
    domain_obj: GitHubDeploymentProjectCreated,
) -> ProjectConfigDto {
    let (owner, repo) = domain_obj.github_owner_repo.get_values();
    ProjectConfigDto {
        project_name: domain_obj.project_name,
        github_personal_token: domain_obj.github_personal_token.to_string(),
        heroku_auth_token: None,
        heroku_app_name: None,
        deployment_source: "github_deployment".to_string(),
        github_owner: owner,
        github_repo: repo,
        developer_count: domain_obj.developer_count.to_u32(),
        working_days_per_week: domain_obj.working_days_per_week.to_f32(),
    }
}
fn to_heroku_release_project_created(
    dto: &ProjectConfigDto,
) -> Result<HerokuReleaseProjectCreated, CreateProjectDtoError> {
    let github_personal_token =
        validate_github_personal_token::perform(Some(dto.github_personal_token.to_string()))?;
    let heroku_app_name = validate_heroku_app_name::perform(dto.heroku_app_name.clone())?;
    let heroku_auth_token = validate_heroku_auth_token::perform(dto.heroku_auth_token.clone())?;
    let github_owner_repo =
        validate_github_owner_repo::perform(format!("{}/{}", dto.github_owner, dto.github_repo))?;
    let developer_count = validate_developer_count::perform(dto.developer_count.to_string())?;
    let working_days_per_week =
        validate_working_days_per_week::perform(dto.working_days_per_week.to_string())?;
    Ok(HerokuReleaseProjectCreated {
        project_name: dto.project_name.clone(),
        github_personal_token,
        github_owner_repo,
        heroku_app_name,
        heroku_auth_token,
        developer_count,
        working_days_per_week,
    })
}

fn from_heroku_release_project_created(
    domain_obj: HerokuReleaseProjectCreated,
) -> ProjectConfigDto {
    let (owner, repo) = domain_obj.github_owner_repo.get_values();
    ProjectConfigDto {
        project_name: domain_obj.project_name,
        github_personal_token: domain_obj.github_personal_token.to_string(),
        heroku_app_name: Some(domain_obj.heroku_app_name.to_string()),
        heroku_auth_token: Some(domain_obj.heroku_auth_token.to_string()),
        deployment_source: "github_deployment".to_string(),
        github_owner: owner,
        github_repo: repo,
        developer_count: domain_obj.developer_count.to_u32(),
        working_days_per_week: domain_obj.working_days_per_week.to_f32(),
    }
}

impl From<ProjectCreated> for ProjectConfigDto {
    fn from(domain_obj: ProjectCreated) -> Self {
        match domain_obj {
            ProjectCreated::GitHubDeployment(domain_obj) => {
                from_github_deployment_project_created(domain_obj)
            }
            ProjectCreated::HerokuRelease(domain_obj) => {
                from_heroku_release_project_created(domain_obj)
            }
        }
    }
}

impl TryFrom<ProjectConfigDto> for ProjectCreated {
    type Error = CreateProjectDtoError;
    fn try_from(dto: ProjectConfigDto) -> Result<Self, Self::Error> {
        match dto.deployment_source.as_str() {
            "github_deployment" => {
                let domain_obj = to_github_deployment_project_created(&dto)?;
                Ok(ProjectCreated::GitHubDeployment(domain_obj))
            }
            "heroku_release" => {
                let domain_obj = to_heroku_release_project_created(&dto)?;
                Ok(ProjectCreated::HerokuRelease(domain_obj))
            }
            _ => Err(CreateProjectDtoError::InvalidDataSource(
                dto.deployment_source,
            )),
        }
    }
}
