use thiserror::Error;

use crate::common_types::ConfigData;

#[derive(Error, Debug)]
pub enum WriteNewConfigError {
    #[error("Cannot write the new config")]
    ConfigFileWriteError
}

pub trait WriteNewConfig {
    fn perform(&self, params: ConfigData) -> Result<(), WriteNewConfigError>;
}
