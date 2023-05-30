use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

// pub fn parse_ymd(s: &str) -> Result<DateTime<Utc>, anyhow::Error> {
//     let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
//     let naive_time = NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
//     let datetime = Utc.from_local_datetime(&naive_time.and_time(time)).unwrap();
//     Ok(datetime)
// }

pub fn parse(s: &str) -> Result<DateTime<Utc>, anyhow::Error> {
    let naive_time = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")?;
    let datetime = Utc.from_local_datetime(&naive_time).unwrap();
    Ok(datetime)
}
