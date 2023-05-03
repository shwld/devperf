use serde::{Serialize, Deserialize};
use std::{collections::HashMap};
use crate::project_creating::{validate_github_personal_token::schema::*, validate_github_owner_repo::schema::*};

// ---------------------------
// Validation step
// ---------------------------

// ProjectConfig validation

#[derive(Debug, Serialize, Deserialize)]
pub enum DeploymentSource {
    GitHubDeployment,
    HerokuRelease,
}
