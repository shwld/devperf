use clap::Subcommand;
use confy::ConfyError;
use std::path::PathBuf;

const CONFY_APP_NAME: &str = "devops-metrics-tools";

#[derive(Subcommand)]
pub enum ConfigAction {
    Get {},
    // Set {
    //     #[clap(long, required = false)]
    //     project: Option<String>,
    // },
}

pub fn get_config_path() -> Result<PathBuf, ConfyError> {
    confy::get_configuration_file_path(CONFY_APP_NAME, None)
}
