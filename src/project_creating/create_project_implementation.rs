use async_trait::async_trait;

use crate::dependencies::project_config_io::writer::interface::ProjectConfigIOWriter;

use super::{
    create_project_internal_types::{
        CreateEvents, CreateGithubProject, CreateHerokuProject, CreateProjectStep,
    },
    create_project_public_types::*,
    dto::ProjectConfigDto,
};

// ---------------------------
// create step
// ---------------------------
const create_github_project: CreateGithubProject =
    |uncreated_project: UncreatedGitHubDeploymentProject| -> GitHubDeploymentProjectCreated {
        GitHubDeploymentProjectCreated {
            project_name: uncreated_project.project_name,
            github_personal_token: uncreated_project.github_personal_token,
            github_owner_repo: uncreated_project.github_owner_repo,
            github_deployment_environment: uncreated_project.github_deployment_environment,
            developer_count: uncreated_project.developer_count,
            working_days_per_week: uncreated_project.working_days_per_week,
        }
    };

const create_heroku_project: CreateHerokuProject =
    |uncreated_project: UncreatedHerokuReleaseProject| -> HerokuReleaseProjectCreated {
        HerokuReleaseProjectCreated {
            project_name: uncreated_project.project_name,
            github_personal_token: uncreated_project.github_personal_token,
            github_owner_repo: uncreated_project.github_owner_repo,
            heroku_app_name: uncreated_project.heroku_app_name,
            heroku_auth_token: uncreated_project.heroku_auth_token,
            developer_count: uncreated_project.developer_count,
            working_days_per_week: uncreated_project.working_days_per_week,
        }
    };

struct CreateProjectStepImpl<T: ProjectConfigIOWriter> {
    project_io_writer: T,
}
#[async_trait]
impl<T: ProjectConfigIOWriter + Sync + Send> CreateProjectStep for CreateProjectStepImpl<T> {
    async fn create_project(
        &self,
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
        self.project_io_writer.write(project_dto).await?;

        Ok(CreateProjectEvent::ProjectCreated(created_project))
    }
}

// ---------------------------
// create events
// ---------------------------
const create_events: CreateEvents =
    |project: CreateProjectEvent| -> Vec<CreateProjectEvent> { vec![project] };

// ---------------------------
// overall workflow
// ---------------------------
pub struct CreateProjectWorkflow<T: ProjectConfigIOWriter + Send + Sync> {
    pub(crate) project_io_writer: T,
}
#[async_trait]
impl<T: ProjectConfigIOWriter + Send + Sync> CreateProject for CreateProjectWorkflow<T> {
    async fn create_project(
        self,
        uncreated_project: UncreatedProject,
    ) -> Result<Vec<CreateProjectEvent>, CreateGithubDeploymentProjectError> {
        let events = create_events(
            CreateProjectStepImpl {
                project_io_writer: self.project_io_writer,
            }
            .create_project(uncreated_project)
            .await?,
        );

        Ok(events)
    }
}
