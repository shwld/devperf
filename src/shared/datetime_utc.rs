use chrono::{DateTime, Utc};

pub fn parse(s: &str) -> Result<DateTime<Utc>, anyhow::Error> {
    DateTime::parse_from_rfc3339(s).map_err(|e| anyhow::anyhow!(e)).map(|x| x.with_timezone(&Utc))
}
