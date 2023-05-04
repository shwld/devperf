pub enum ProjectAccessToken<T> {
  UseGlobal,
  Override(T),
}

pub enum DeploymentSource {
    GitHubDeployment,
    HerokuRelease,
}
