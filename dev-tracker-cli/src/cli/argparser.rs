use chrono::offset::TimeZone;
use chrono::{DateTime, Local, NaiveDateTime, Utc};

use super::errors::ArgParserError;

pub fn parse_datetime(arg: &str) -> Result<DateTime<Utc>, ArgParserError> {
    println!("ARG: {}", arg);
    let datetime = NaiveDateTime::parse_from_str(arg, "%Y-%m-%dT%H:%M")?;
    let datetime = match Local.from_local_datetime(&datetime) {
        chrono::LocalResult::Single(datetime) => datetime,
        _ => panic!("Failed to convert local datetime into UTC datetime."),
    };
    let datetime: DateTime<Utc> = DateTime::from(datetime);

    Ok(datetime)
}
