use cranenum::Cranenum;

use crate::{project_creating::{validate_github_personal_token::{self, schema::ValidateGitHubPersonalTokenError}, validate_github_owner_repo::{self, schema::ValidateGitHubOwnerRepoError}, validate_developer_count::{self, schema::ValidateDeveloperCountError}, validate_working_days_per_week::{self, schema::ValidateWorkingDaysPerWeekError}}, dependencies::write_new_config::interface::{WriteNewConfigData, ProjectConfig}};

use super::schema::GitHubDeploymentProjectCreated;

pub type GitHubDeploymentProjectCreatedDto = WriteNewConfigData;

#[derive(Cranenum)]
pub enum ToGitHubDeploymentProjectCreatedError {
  ValidateGitHubPersonalTokenError(ValidateGitHubPersonalTokenError),
  ValidateGitHubOwnerRepoError(ValidateGitHubOwnerRepoError),
  ValidateDeveloperCountError(ValidateDeveloperCountError),
  ValidateWorkingDaysPerWeekError(ValidateWorkingDaysPerWeekError),
}

impl GitHubDeploymentProjectCreatedDto {
    pub fn to_git_hub_deployment_project_created(dto: &WriteNewConfigData) -> Result<GitHubDeploymentProjectCreated, ToGitHubDeploymentProjectCreatedError> {
        let github_personal_token = validate_github_personal_token::workflow::perform(Some(dto.github_personal_token.to_string()))?;
        let github_owner_repo = validate_github_owner_repo::workflow::perform(format!("{}/{}", dto.project_config.github_owner, dto.project_config.github_repo))?;
        let developer_count = validate_developer_count::workflow::perform(dto.project_config.developer_count.to_string())?;
        let working_days_per_week = validate_working_days_per_week::workflow::perform(dto.project_config.working_days_per_week.to_string())?;
        Ok(GitHubDeploymentProjectCreated {
            project_name: dto.project_name.to_string(),
            github_personal_token: github_personal_token,
            github_owner_repo: github_owner_repo,
            developer_count: developer_count,
            working_days_per_week: working_days_per_week,
        })
    }

    pub fn from_git_hub_deployment_project_created(domain_obj: GitHubDeploymentProjectCreated) -> WriteNewConfigData {
        let (owner, repo) = domain_obj.github_owner_repo.get_values();
        GitHubDeploymentProjectCreatedDto {
            project_name: domain_obj.project_name,
            github_personal_token: domain_obj.github_personal_token.to_string(),
            project_config: ProjectConfig {
                deployment_source: "github_deployment".to_string(),
                github_owner: owner,
                github_repo: repo,
                developer_count: domain_obj.developer_count.to_u32(),
                working_days_per_week: domain_obj.working_days_per_week.to_f32(),
            },
        }
    }
}
