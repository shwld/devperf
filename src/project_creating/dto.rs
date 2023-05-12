use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common_types::{
    deployment_source::DeploymentSource,
    developer_count::{ValidateDeveloperCountError, ValidatedDeveloperCount},
    github_deployment_environment::{
        ValidateGitHubDeploymentEnvironmentError, ValidatedGitHubDeploymentEnvironment,
    },
    github_owner_repo::{ValidateGitHubOwnerRepoError, ValidatedGitHubOwnerRepo},
    github_personal_token::{ValidateGitHubPersonalTokenError, ValidatedGitHubPersonalToken},
    heroku_app_name::{ValidateHerokuAppNameError, ValidatedHerokuAppName},
    heroku_auth_token::{ValidateHerokuAuthTokenError, ValidatedHerokuAuthToken},
    working_days_per_week::{ValidateWorkingDaysPerWeekError, ValidatedWorkingDaysPerWeek},
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
    pub github_deployment_environment: Option<String>,
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
    #[error("GitHub deployment environment is invalid")]
    GitHubDeploymentEnvironment(#[from] ValidateGitHubDeploymentEnvironmentError),
    #[error("GitHub developer count is invalid")]
    DeveloperCount(#[from] ValidateDeveloperCountError),
    #[error("GitHub working days per week is invalid")]
    WorkingDaysPerWeek(#[from] ValidateWorkingDaysPerWeekError),
    #[error("Heroku authorization token is invalid")]
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
        ValidatedGitHubPersonalToken::new(Some(dto.github_personal_token.clone()))?;
    let github_owner_repo =
        ValidatedGitHubOwnerRepo::new(format!("{}/{}", dto.github_owner, dto.github_repo))?;
    let github_deployment_environment =
        ValidatedGitHubDeploymentEnvironment::new(dto.github_deployment_environment.clone())?;
    let developer_count = ValidatedDeveloperCount::new(dto.developer_count.to_string())?;
    let working_days_per_week =
        ValidatedWorkingDaysPerWeek::new(dto.working_days_per_week.to_string())?;
    Ok(GitHubDeploymentProjectCreated {
        project_name: dto.project_name.to_string(),
        github_personal_token,
        github_owner_repo,
        github_deployment_environment,
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
        deployment_source: DeploymentSource::GitHubDeployment.value(),
        github_owner: owner,
        github_repo: repo,
        github_deployment_environment: Some(domain_obj.github_deployment_environment.to_string()),
        developer_count: domain_obj.developer_count.to_u32(),
        working_days_per_week: domain_obj.working_days_per_week.to_f32(),
    }
}
fn to_heroku_release_project_created(
    dto: &ProjectConfigDto,
) -> Result<HerokuReleaseProjectCreated, CreateProjectDtoError> {
    let github_personal_token =
        ValidatedGitHubPersonalToken::new(Some(dto.github_personal_token.to_string()))?;
    let heroku_app_name = ValidatedHerokuAppName::new(dto.heroku_app_name.clone())?;
    let heroku_auth_token = ValidatedHerokuAuthToken::new(dto.heroku_auth_token.clone())?;
    let github_owner_repo =
        ValidatedGitHubOwnerRepo::new(format!("{}/{}", dto.github_owner, dto.github_repo))?;
    let developer_count = ValidatedDeveloperCount::new(dto.developer_count.to_string())?;
    let working_days_per_week =
        ValidatedWorkingDaysPerWeek::new(dto.working_days_per_week.to_string())?;
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
        github_owner: owner,
        github_repo: repo,
        github_deployment_environment: None,
        heroku_app_name: Some(domain_obj.heroku_app_name.to_string()),
        heroku_auth_token: Some(domain_obj.heroku_auth_token.to_string()),
        deployment_source: DeploymentSource::HerokuRelease.value(),
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
        if dto.deployment_source.as_str() == DeploymentSource::GitHubDeployment.value() {
            let domain_obj = to_github_deployment_project_created(&dto)?;
            Ok(ProjectCreated::GitHubDeployment(domain_obj))
        } else if dto.deployment_source.as_str() == DeploymentSource::HerokuRelease.value() {
            let domain_obj = to_heroku_release_project_created(&dto)?;
            Ok(ProjectCreated::HerokuRelease(domain_obj))
        } else {
            Err(CreateProjectDtoError::InvalidDataSource(
                dto.deployment_source,
            ))
        }
    }
}
