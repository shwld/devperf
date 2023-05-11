use async_trait::async_trait;

use super::create_project_public_types::*;

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

fn create_project(uncreated_project: UncreatedProject) -> CreateProjectEvent {
    match uncreated_project {
        UncreatedProject::GitHubDeployment(uncreated_project) => {
            let project = create_github_project(uncreated_project);
            CreateProjectEvent::ProjectCreated(ProjectCreated::GitHubDeployment(project))
        }
        UncreatedProject::HerokuRelease(uncreated_project) => {
            let project = create_heroku_project(uncreated_project);
            CreateProjectEvent::ProjectCreated(ProjectCreated::HerokuRelease(project))
        }
    }
}

// ---------------------------
// create events
// ---------------------------
fn create_events(
    project: CreateProjectEvent,
) -> Result<Vec<CreateProjectEvent>, CreateGithubDeploymentProjectError> {
    Ok(vec![project])
}

// ---------------------------
// overall workflow
// ---------------------------
pub struct CreateProject {}
#[async_trait]
impl CreateProjectWorkflow for CreateProject {
    async fn create_project(
        &self,
        uncreated_project: UncreatedProject,
    ) -> Result<Vec<CreateProjectEvent>, CreateGithubDeploymentProjectError> {
        let project = create_project(uncreated_project);
        let events = create_events(project)?;
        Ok(events)
    }
}
