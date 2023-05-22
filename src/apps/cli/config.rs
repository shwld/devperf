use clap::Subcommand;
use confy::ConfyError;
use std::path::PathBuf;

pub const CONFY_APP_NAME: &str = "devperf";

#[derive(Subcommand)]
pub enum ConfigAction {
    Get {},
}

pub fn get_config_path() -> Result<PathBuf, ConfyError> {
    confy::get_configuration_file_path(CONFY_APP_NAME, None)
}
