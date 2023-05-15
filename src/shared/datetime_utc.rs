use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone, Utc};

pub fn parse(s: &str) -> Result<DateTime<Utc>, anyhow::Error> {
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let naive_since = NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
    let datetime = Utc
        .from_local_datetime(&naive_since.and_time(time))
        .unwrap();
    Ok(datetime)
}
