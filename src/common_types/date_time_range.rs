use std::cmp::Ordering;

use chrono::{DateTime, TimeZone, Utc};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct DateTimeRange<T: TimeZone = Utc> {
    pub(super) since: DateTime<T>,
    pub(super) until: DateTime<T>,
}

#[derive(Debug, Error, Clone)]
pub enum ValidateDateTimeRangeError {
    #[error("{0}")]
    Invalid(String),
}

impl DateTimeRange {
    pub fn new<T: TimeZone>(
        since: &DateTime<T>,
        until: &DateTime<T>,
    ) -> Result<Self, ValidateDateTimeRangeError> {
        match since.cmp(until) {
            Ordering::Less => Ok(DateTimeRange { since, until }),
            Ordering::Equal => Err(ValidateDateTimeRangeError::Invalid(
                "Since and until are equal".to_string(),
            )),
            Ordering::Greater => Err(ValidateDateTimeRangeError::Invalid(
                "Since is greater than until".to_string(),
            )),
        }
    }
}
