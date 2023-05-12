use async_trait::async_trait;

use crate::dependencies::project_config_io::writer::interface::ProjectConfigIOWriter;

use super::{create_project_public_types::*, dto::ProjectConfigDto};

// ---------------------------
// create step
// ---------------------------
fn create_github_project(
    uncreated_project: UncreatedGitHubDeploymentProject,
) -> GitHubDeploymentProjectCreated {
    GitHubDeploymentProjectCreated {
        project_name: uncreated_project.project_name,
        github_personal_token: uncreated_project.github_personal_token,
        github_owner_repo: uncreated_project.github_owner_repo,
        github_deployment_environment: uncreated_project.github_deployment_environment,
        developer_count: uncreated_project.developer_count,
        working_days_per_week: uncreated_project.working_days_per_week,
    }
}

fn create_heroku_project(
    uncreated_project: UncreatedHerokuReleaseProject,
) -> HerokuReleaseProjectCreated {
    HerokuReleaseProjectCreated {
        project_name: uncreated_project.project_name,
        github_personal_token: uncreated_project.github_personal_token,
        github_owner_repo: uncreated_project.github_owner_repo,
        heroku_app_name: uncreated_project.heroku_app_name,
        heroku_auth_token: uncreated_project.heroku_auth_token,
        developer_count: uncreated_project.developer_count,
        working_days_per_week: uncreated_project.working_days_per_week,
    }
}

async fn create_project<T: ProjectConfigIOWriter>(
    project_io_writer: &T,
    uncreated_project: UncreatedProject,
) -> Result<CreateProjectEvent, CreateGithubDeploymentProjectError> {
    let created_project = match uncreated_project {
        UncreatedProject::GitHubDeployment(uncreated_project) => {
            let project = create_github_project(uncreated_project);
            ProjectCreated::GitHubDeployment(project)
        }
        UncreatedProject::HerokuRelease(uncreated_project) => {
            let project = create_heroku_project(uncreated_project);
            ProjectCreated::HerokuRelease(project)
        }
    };

    let project_dto: ProjectConfigDto = created_project.clone().into();
    project_io_writer.write(project_dto).await?;

    Ok(CreateProjectEvent::ProjectCreated(created_project))
}

// ---------------------------
// create events
// ---------------------------
fn create_events(project: CreateProjectEvent) -> Vec<CreateProjectEvent> {
    vec![project]
}

// ---------------------------
// overall workflow
// ---------------------------
pub struct CreateProjectWorkflow<T: ProjectConfigIOWriter + Send + Sync> {
    pub(crate) project_io_writer: T,
}
#[async_trait]
impl<T: ProjectConfigIOWriter + Send + Sync> CreateProject for CreateProjectWorkflow<T> {
    async fn create_project(
        &self,
        uncreated_project: UncreatedProject,
    ) -> Result<Vec<CreateProjectEvent>, CreateGithubDeploymentProjectError> {
        let project = create_project(&self.project_io_writer, uncreated_project).await?;
        let events = create_events(project);

        Ok(events)
    }
}
