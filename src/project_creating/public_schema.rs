use super::{validate_github_personal_token::schema::ValidatedGitHubPersonalToken, validate_github_owner_repo::schema::ValidatedGitHubOwnerRepo, validate_developer_count::schema::ValidatedDeveloperCount, validate_working_days_per_week::schema::ValidatedWorkingDaysPerWeek};

pub enum ProjectAccessToken<T> {
  UseGlobal,
  Override(T),
}

#[derive(Clone)]
pub struct GitHubDeploymentProjectConfig {
    pub project_name: String,
    pub github_personal_token: ValidatedGitHubPersonalToken,
    pub github_owner_repo: ValidatedGitHubOwnerRepo,
    pub developer_count: ValidatedDeveloperCount,
    pub working_days_per_week: ValidatedWorkingDaysPerWeek,
}

pub enum ProjectConfig {
    GitHubDeployment(GitHubDeploymentProjectConfig),
    HerokuRelease,
}
