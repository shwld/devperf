use std::{cmp::Ordering, ops::RangeInclusive};

use chrono::{DateTime, Datelike, Duration, NaiveTime, TimeZone, Utc, Weekday};
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
        self.until.signed_duration_since(self.since).num_days() + 1
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
        let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

        // If since is Holiday, then we need to start from Monday
        let since = match self.since.date_naive().weekday() {
            Weekday::Sat => self.since.date_naive() + Duration::days(2),
            Weekday::Sun => self.since.date_naive() + Duration::days(1),
            _ => self.since.date_naive(),
        };
        let since_date = since.week(Weekday::Mon).first_day();
        let since = Utc.from_local_datetime(&since_date.and_time(time)).unwrap();
        let until_date = self.until.date_naive().week(Weekday::Mon).first_day();
        let until = Utc.from_local_datetime(&until_date.and_time(time)).unwrap();
        DateTimeRange {
            since,
            until,
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
        if self.since > self.until {
            return None;
        }
        let result = Some(self.since);
        self.since += self.iter_duration.expect("iter_duration is None");
        result
    }
}
