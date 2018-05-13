extern crate chrono;

use chrono::NaiveDateTime;
use chrono::DateTime;
use chrono::Utc;
use chrono::ParseError;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;

pub fn parse(date: &str) -> Result<DateTime<Utc>, ParseError> {

    let current = SystemTime::now();
    let epoch = current.duration_since(UNIX_EPOCH).unwrap();

    let naive = NaiveDateTime::from_timestamp(epoch.as_secs() as i64, epoch.subsec_nanos());

    Ok(DateTime::from_utc(naive, Utc))
}
