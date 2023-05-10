use crate::project_parameter_validating::{
    validate_developer_count::ValidatedDeveloperCount,
    validate_github_owner_repo::ValidatedGitHubOwnerRepo,
    validate_github_personal_token::ValidatedGitHubPersonalToken,
    validate_heroku_app_name::ValidatedHerokuAppName,
    validate_heroku_auth_token::ValidatedHerokuApiToken,
    validate_working_days_per_week::ValidatedWorkingDaysPerWeek,
};

#[derive(Clone)]
pub struct GitHubDeploymentProjectConfig {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}

#[derive(Clone)]
pub struct HerokuReleaseProjectConfig {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub heroku_app_name: ValidatedHerokuAppName,
    pub heroku_api_token: ValidatedHerokuApiToken,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}
