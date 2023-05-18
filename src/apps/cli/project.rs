use anyhow::Result;
use clap::Subcommand;

use super::initializer;

#[derive(Subcommand)]
pub enum ProjectAction {
    Add {},
}

pub async fn add() -> Result<()> {
    initializer::add_project::perform().await
}
