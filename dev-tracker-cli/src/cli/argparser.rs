use chrono::offset::TimeZone;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use super::errors::ArgParserError;

pub fn parse_datetime(arg: &str) -> Result<DateTime<Utc>, ArgParserError> {
    let datetime = NaiveDateTime::parse_from_str(arg, "%Y-%m-%dT%H:%M")?;

    let datetime = match Local.from_local_datetime(&datetime) {
        chrono::LocalResult::Single(datetime) => datetime,
        _ => panic!("Failed to convert local datetime into UTC datetime."),
    };

    let datetime: DateTime<Utc> = DateTime::from(datetime);

    Ok(datetime)
}

pub fn parse_date(arg: &str) -> Result<DateTime<Utc>, ArgParserError> {
    let date = NaiveDate::parse_from_str(arg, "%d-%m-%Y")?;
    let time = NaiveTime::from_hms_opt(12, 0, 0)
        .expect("should always be able to create a time of midday");
    let datetime = NaiveDateTime::new(date, time);

    let datetime = match Local.from_local_datetime(&datetime) {
        chrono::LocalResult::Single(datetime) => datetime,
        _ => panic!("Failed to convert local datetime into UTC datetime."),
    };

    let datetime: DateTime<Utc> = DateTime::from(datetime);

    Ok(datetime)
}
