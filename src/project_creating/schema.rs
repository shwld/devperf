use super::create_github_deployment_project::dto::GitHubDeploymentProjectCreatedDto;

pub enum ProjectAccessToken<T> {
  UseGlobal,
  Override(T),
}

pub enum DeploymentSource {
    GitHubDeployment,
    HerokuRelease,
}
