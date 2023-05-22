use std::{cmp::Ordering, ops::RangeInclusive};

use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeRange {
    pub(super) since: DateTime<Utc>,
    pub(super) until: DateTime<Utc>,
    #[serde(skip)]
    iter_duration: Option<Duration>,
}

#[derive(Debug, Error, Clone)]
pub enum ValidateDateTimeRangeError {
    #[error("{0}")]
    Invalid(String),
}

impl DateTimeRange {
    pub fn new(
        since: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> Result<Self, ValidateDateTimeRangeError> {
        match since.cmp(&until) {
            Ordering::Less => Ok(DateTimeRange {
                since,
                until,
                iter_duration: None,
            }),
            Ordering::Equal => Err(ValidateDateTimeRangeError::Invalid(
                "Since and until are equal".to_string(),
            )),
            Ordering::Greater => Err(ValidateDateTimeRangeError::Invalid(
                "Since is greater than until".to_string(),
            )),
        }
    }

    pub fn num_days(&self) -> i64 {
        self.until.signed_duration_since(self.since).num_days()
    }

    pub fn is_include(&self, datetime: &DateTime<Utc>) -> bool {
        self.since <= *datetime && *datetime <= self.until
    }

    pub fn days_iter(&self) -> DateTimeRange {
        DateTimeRange {
            since: self.since,
            until: self.until,
            iter_duration: Some(Duration::days(1)),
        }
    }

    pub fn weeks_iter(&self) -> DateTimeRange {
        DateTimeRange {
            since: self.since,
            until: self.until,
            iter_duration: Some(Duration::weeks(1)),
        }
    }

    pub fn months_iter(&self) -> RangeInclusive<u32> {
        self.since.month()..=self.until.month()
    }
}

impl Iterator for DateTimeRange {
    type Item = DateTime<Utc>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.since >= self.until {
            return None;
        }
        let result = Some(self.since);
        self.since += self.iter_duration.expect("iter_duration is None");
        result
    }
}
