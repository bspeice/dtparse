extern crate chrono;
extern crate dtparse;

use chrono::DateTime;
use chrono::Utc;
use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::NaiveDateTime;

use dtparse::parse;

macro_rules! ymd_test {
    ($date: expr, $year: expr, $month: expr, $day: expr) => {
        let nd = NaiveDate::from_ymd($year, $month, $day);
        let nt = NaiveTime::from_hms(0, 0, 0);
        let dt = NaiveDateTime::new(nd, nt);
        let utc_dt = DateTime::from_utc(dt, Utc);

        let parsed = parse($date);

        println!("{:?}", parsed);
        assert!(parsed == Ok(utc_dt));
    };
}

#[test]
fn test_basic() {
    ymd_test!("2014 January 19", 2014, 1, 19);
}