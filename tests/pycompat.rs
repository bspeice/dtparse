
extern crate chrono;

use chrono::Datelike;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::Timelike;
use std::collections::HashMap;

extern crate dtparse;

use dtparse::Parser;
use dtparse::ParserInfo;

struct PyDateTime {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    micros: u32,
    tzo: Option<i32>
}

fn parse_and_assert(
    pdt: PyDateTime,
    info: ParserInfo,
    s: &str,
    dayfirst: Option<bool>,
    yearfirst: Option<bool>,
    fuzzy: bool,
    fuzzy_with_tokens: bool,
    default: Option<&NaiveDateTime>,
    ignoretz: bool,
    tzinfos: HashMap<String, i32>,
) {

    let mut parser = Parser::new(info);
    let rs_parsed = parser.parse(
        s,
        dayfirst,
        yearfirst,
        fuzzy,
        fuzzy_with_tokens,
        default,
        ignoretz,
        tzinfos).expect(&format!("Unable to parse date in Rust '{}'", s));

    assert_eq!(pdt.year, rs_parsed.0.year(), "Year mismatch for '{}'", s);
    assert_eq!(pdt.month, rs_parsed.0.month(), "Month mismatch for '{}'", s);
    assert_eq!(pdt.day, rs_parsed.0.day(), "Day mismatch for '{}'", s);
    assert_eq!(pdt.hour, rs_parsed.0.hour(), "Hour mismatch for '{}'", s);
    assert_eq!(pdt.minute, rs_parsed.0.minute(), "Minute mismatch f'or' {}", s);
    assert_eq!(pdt.second, rs_parsed.0.second(), "Second mismatch for '{}'", s);
    assert_eq!(pdt.micros, rs_parsed.0.timestamp_subsec_micros(), "Microsecond mismatch for {}", s);
    assert_eq!(pdt.tzo, rs_parsed.1.map(|u| u.local_minus_utc()), "Timezone Offset mismatch for {}", s);
}

fn parse_and_assert_simple(
    pdt: PyDateTime,
    s: &str,
) {
    let rs_parsed = dtparse::parse(s).expect(&format!("Unable to parse date in Rust '{}'", s));
    assert_eq!(pdt.year, rs_parsed.0.year(), "Year mismatch for {}", s);
    assert_eq!(pdt.month, rs_parsed.0.month(), "Month mismatch for {}", s);
    assert_eq!(pdt.day, rs_parsed.0.day(), "Day mismatch for {}", s);
    assert_eq!(pdt.hour, rs_parsed.0.hour(), "Hour mismatch for {}", s);
    assert_eq!(pdt.minute, rs_parsed.0.minute(), "Minute mismatch for {}", s);
    assert_eq!(pdt.second, rs_parsed.0.second(), "Second mismatch for {}", s);
    assert_eq!(pdt.micros, rs_parsed.0.timestamp_subsec_micros(), "Microsecond mismatch for {}", s);
    assert_eq!(pdt.tzo, rs_parsed.1.map(|u| u.local_minus_utc()), "Timezone Offset mismatch for {}", s);
}

fn parse_fuzzy_and_assert(
    pdt: PyDateTime,
    ptokens: Option<Vec<String>>,
    info: ParserInfo,
    s: &str,
    dayfirst: Option<bool>,
    yearfirst: Option<bool>,
    fuzzy: bool,
    fuzzy_with_tokens: bool,
    default: Option<&NaiveDateTime>,
    ignoretz: bool,
    tzinfos: HashMap<String, i32>,
) {

    let mut parser = Parser::new(info);
    let rs_parsed = parser.parse(
        s,
        dayfirst,
        yearfirst,
        fuzzy,
        fuzzy_with_tokens,
        default,
        ignoretz,
        tzinfos).expect(&format!("Unable to parse date in Rust '{}'", s));

    assert_eq!(pdt.year, rs_parsed.0.year(), "Year mismatch for '{}'", s);
    assert_eq!(pdt.month, rs_parsed.0.month(), "Month mismatch for '{}'", s);
    assert_eq!(pdt.day, rs_parsed.0.day(), "Day mismatch for '{}'", s);
    assert_eq!(pdt.hour, rs_parsed.0.hour(), "Hour mismatch for '{}'", s);
    assert_eq!(pdt.minute, rs_parsed.0.minute(), "Minute mismatch f'or' {}", s);
    assert_eq!(pdt.second, rs_parsed.0.second(), "Second mismatch for '{}'", s);
    assert_eq!(pdt.micros, rs_parsed.0.timestamp_subsec_micros(), "Microsecond mismatch for {}", s);
    assert_eq!(pdt.tzo, rs_parsed.1.map(|u| u.local_minus_utc()), "Timezone Offset mismatch for {}", s);
    assert_eq!(ptokens, rs_parsed.2, "Fuzzy mismatch for {}", s);
}

macro_rules! rs_tzinfo_map {
    () => ({
        let mut h = HashMap::new();
        h.insert("BRST".to_owned(), -10800);
        h
    });
}

#[test]
fn test_parse_default0() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 28,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Thu Sep 25 10:36:28", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default1() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 28,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Sep 10:36:28", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default2() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 28,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10:36:28", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default3() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10:36", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default4() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Sep 2003", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default5() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Sep", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default6() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "2003", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default7() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 28,
        micros: 500000, tzo: None
    };
    parse_and_assert(pdt, info, "10h36m28.5s", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default8() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 28,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10h36m28s", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default9() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10h36m", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default10() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10h", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default11() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10 h 36", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default12() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 30,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10 h 36.5", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default13() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 36, second: 5,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "36 m 5", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default14() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 36, second: 5,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "36 m 5 s", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default15() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 36, second: 5,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "36 m 05", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default16() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 36, second: 5,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "36 m 05 s", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default17() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10h am", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default18() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 22, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10h pm", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default19() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10am", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default20() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 22, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10pm", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default21() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10:00 am", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default22() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 22, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10:00 pm", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default23() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10:00am", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default24() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 22, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10:00pm", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default25() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10:00a.m", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default26() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 22, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10:00p.m", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default27() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10:00a.m.", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default28() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 22, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "10:00p.m.", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default29() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 10, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "October", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default30() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2000, month: 12, day: 31,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "31-Dec-00", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default31() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 1, second: 2,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "0:01:02", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default32() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 1, second: 2,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "12h 01m02s am", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default33() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 12, minute: 8, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "12:08 PM", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default34() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 1, minute: 2, second: 3,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "01h02m03", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default35() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 1, minute: 2, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "01h02", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default36() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 1, minute: 0, second: 2,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "01h02s", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default37() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 1, second: 2,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "01m02", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default38() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 2, minute: 1, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "01m02h", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default39() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2004, month: 4, day: 10,
        hour: 11, minute: 30, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "2004 10 Apr 11h30m", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default40() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 3,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Sep 03", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default41() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Sep of 03", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default42() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2017, month: 11, day: 25,
        hour: 2, minute: 17, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "02:17NOV2017", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default43() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 28,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Thu Sep 10:36:28", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default44() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 28,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Thu 10:36:28", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default45() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 10, day: 1,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Wed", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_default46() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2003, month: 10, day: 1,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Wednesday", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_simple0() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 28,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "Thu Sep 25 10:36:28 2003");
}

#[test]
fn test_parse_simple1() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "Thu Sep 25 2003");
}

#[test]
fn test_parse_simple2() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 49, second: 41,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "2003-09-25T10:49:41");
}

#[test]
fn test_parse_simple3() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 49, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "2003-09-25T10:49");
}

#[test]
fn test_parse_simple4() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "2003-09-25T10");
}

#[test]
fn test_parse_simple5() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "2003-09-25");
}

#[test]
fn test_parse_simple6() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 49, second: 41,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "20030925T104941");
}

#[test]
fn test_parse_simple7() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 49, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "20030925T1049");
}

#[test]
fn test_parse_simple8() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "20030925T10");
}

#[test]
fn test_parse_simple9() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "20030925");
}

#[test]
fn test_parse_simple10() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 49, second: 41,
        micros: 502000, tzo: None,
    };
    parse_and_assert_simple(pdt, "2003-09-25 10:49:41,502");
}

#[test]
fn test_parse_simple11() {
    let pdt = PyDateTime {
        year: 1997, month: 9, day: 2,
        hour: 9, minute: 8, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "199709020908");
}

#[test]
fn test_parse_simple12() {
    let pdt = PyDateTime {
        year: 1997, month: 9, day: 2,
        hour: 9, minute: 8, second: 7,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "19970902090807");
}

#[test]
fn test_parse_simple13() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "2003-09-25");
}

#[test]
fn test_parse_simple14() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "09-25-2003");
}

#[test]
fn test_parse_simple15() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "25-09-2003");
}

#[test]
fn test_parse_simple16() {
    let pdt = PyDateTime {
        year: 2003, month: 10, day: 9,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "10-09-2003");
}

#[test]
fn test_parse_simple17() {
    let pdt = PyDateTime {
        year: 2003, month: 10, day: 9,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "10-09-03");
}

#[test]
fn test_parse_simple18() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "2003.09.25");
}

#[test]
fn test_parse_simple19() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "09.25.2003");
}

#[test]
fn test_parse_simple20() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "25.09.2003");
}

#[test]
fn test_parse_simple21() {
    let pdt = PyDateTime {
        year: 2003, month: 10, day: 9,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "10.09.2003");
}

#[test]
fn test_parse_simple22() {
    let pdt = PyDateTime {
        year: 2003, month: 10, day: 9,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "10.09.03");
}

#[test]
fn test_parse_simple23() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "2003/09/25");
}

#[test]
fn test_parse_simple24() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "09/25/2003");
}

#[test]
fn test_parse_simple25() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "25/09/2003");
}

#[test]
fn test_parse_simple26() {
    let pdt = PyDateTime {
        year: 2003, month: 10, day: 9,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "10/09/2003");
}

#[test]
fn test_parse_simple27() {
    let pdt = PyDateTime {
        year: 2003, month: 10, day: 9,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "10/09/03");
}

#[test]
fn test_parse_simple28() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "2003 09 25");
}

#[test]
fn test_parse_simple29() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "09 25 2003");
}

#[test]
fn test_parse_simple30() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "25 09 2003");
}

#[test]
fn test_parse_simple31() {
    let pdt = PyDateTime {
        year: 2003, month: 10, day: 9,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "10 09 2003");
}

#[test]
fn test_parse_simple32() {
    let pdt = PyDateTime {
        year: 2003, month: 10, day: 9,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "10 09 03");
}

#[test]
fn test_parse_simple33() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "25 09 03");
}

#[test]
fn test_parse_simple34() {
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "03 25 Sep");
}

#[test]
fn test_parse_simple35() {
    let pdt = PyDateTime {
        year: 2025, month: 9, day: 3,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "25 03 Sep");
}

#[test]
fn test_parse_simple36() {
    let pdt = PyDateTime {
        year: 1976, month: 7, day: 4,
        hour: 0, minute: 1, second: 2,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "  July   4 ,  1976   12:01:02   am  ");
}

#[test]
fn test_parse_simple37() {
    let pdt = PyDateTime {
        year: 1996, month: 7, day: 10,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "Wed, July 10, '96");
}

#[test]
fn test_parse_simple38() {
    let pdt = PyDateTime {
        year: 1996, month: 7, day: 10,
        hour: 12, minute: 8, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "1996.July.10 AD 12:08 PM");
}

#[test]
fn test_parse_simple39() {
    let pdt = PyDateTime {
        year: 1976, month: 7, day: 4,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "July 4, 1976");
}

#[test]
fn test_parse_simple40() {
    let pdt = PyDateTime {
        year: 1976, month: 7, day: 4,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "7 4 1976");
}

#[test]
fn test_parse_simple41() {
    let pdt = PyDateTime {
        year: 1976, month: 7, day: 4,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "4 jul 1976");
}

#[test]
fn test_parse_simple42() {
    let pdt = PyDateTime {
        year: 1976, month: 7, day: 4,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "7-4-76");
}

#[test]
fn test_parse_simple43() {
    let pdt = PyDateTime {
        year: 1976, month: 7, day: 4,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "19760704");
}

#[test]
fn test_parse_simple44() {
    let pdt = PyDateTime {
        year: 1976, month: 7, day: 4,
        hour: 0, minute: 1, second: 2,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "0:01:02 on July 4, 1976");
}

#[test]
fn test_parse_simple45() {
    let pdt = PyDateTime {
        year: 1976, month: 7, day: 4,
        hour: 0, minute: 1, second: 2,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "0:01:02 on July 4, 1976");
}

#[test]
fn test_parse_simple46() {
    let pdt = PyDateTime {
        year: 1976, month: 7, day: 4,
        hour: 0, minute: 1, second: 2,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "July 4, 1976 12:01:02 am");
}

#[test]
fn test_parse_simple47() {
    let pdt = PyDateTime {
        year: 1995, month: 1, day: 2,
        hour: 4, minute: 24, second: 27,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "Mon Jan  2 04:24:27 1995");
}

#[test]
fn test_parse_simple48() {
    let pdt = PyDateTime {
        year: 1995, month: 4, day: 4,
        hour: 0, minute: 22, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "04.04.95 00:22");
}

#[test]
fn test_parse_simple49() {
    let pdt = PyDateTime {
        year: 1999, month: 1, day: 1,
        hour: 11, minute: 23, second: 34,
        micros: 578000, tzo: None,
    };
    parse_and_assert_simple(pdt, "Jan 1 1999 11:23:34.578");
}

#[test]
fn test_parse_simple50() {
    let pdt = PyDateTime {
        year: 1995, month: 4, day: 4,
        hour: 12, minute: 22, second: 12,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "950404 122212");
}

#[test]
fn test_parse_simple51() {
    let pdt = PyDateTime {
        year: 2001, month: 5, day: 3,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "3rd of May 2001");
}

#[test]
fn test_parse_simple52() {
    let pdt = PyDateTime {
        year: 2001, month: 3, day: 5,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "5th of March 2001");
}

#[test]
fn test_parse_simple53() {
    let pdt = PyDateTime {
        year: 2003, month: 5, day: 1,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "1st of May 2003");
}

#[test]
fn test_parse_simple54() {
    let pdt = PyDateTime {
        year: 99, month: 1, day: 1,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "0099-01-01T00:00:00");
}

#[test]
fn test_parse_simple55() {
    let pdt = PyDateTime {
        year: 31, month: 1, day: 1,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "0031-01-01T00:00:00");
}

#[test]
fn test_parse_simple56() {
    let pdt = PyDateTime {
        year: 2008, month: 2, day: 27,
        hour: 21, minute: 26, second: 1,
        micros: 123456, tzo: None,
    };
    parse_and_assert_simple(pdt, "20080227T21:26:01.123456789");
}

#[test]
fn test_parse_simple57() {
    let pdt = PyDateTime {
        year: 2017, month: 11, day: 13,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "13NOV2017");
}

#[test]
fn test_parse_simple58() {
    let pdt = PyDateTime {
        year: 3, month: 3, day: 4,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "0003-03-04");
}

#[test]
fn test_parse_simple59() {
    let pdt = PyDateTime {
        year: 31, month: 12, day: 30,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "December.0031.30");
}

#[test]
fn test_parse_simple60() {
    let pdt = PyDateTime {
        year: 2007, month: 9, day: 1,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "090107");
}

#[test]
fn test_parse_simple61() {
    let pdt = PyDateTime {
        year: 2015, month: 5, day: 15,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert_simple(pdt, "2015-15-May");
}

#[test]
fn test_parse_tzinfo0() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 28,
        micros: 0, tzo: Some(-10800),
    };
    parse_and_assert(pdt, info, "Thu Sep 25 10:36:28 BRST 2003", None, None, false, false,
                     None, false, rs_tzinfo_map!());
}

#[test]
fn test_parse_tzinfo1() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 28,
        micros: 0, tzo: Some(-10800),
    };
    parse_and_assert(pdt, info, "2003 10:36:28 BRST 25 Sep Thu", None, None, false, false,
                     None, false, rs_tzinfo_map!());
}

#[test]
fn test_parse_offset0() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 49, second: 41,
        micros: 0, tzo: Some(-10800),
    };
    parse_and_assert(pdt, info, "Thu, 25 Sep 2003 10:49:41 -0300", None, None, false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_offset1() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 49, second: 41,
        micros: 500000, tzo: Some(-10800),
    };
    parse_and_assert(pdt, info, "2003-09-25T10:49:41.5-03:00", None, None, false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_offset2() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 49, second: 41,
        micros: 0, tzo: Some(-10800),
    };
    parse_and_assert(pdt, info, "2003-09-25T10:49:41-03:00", None, None, false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_offset3() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 49, second: 41,
        micros: 500000, tzo: Some(-10800),
    };
    parse_and_assert(pdt, info, "20030925T104941.5-0300", None, None, false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_offset4() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 49, second: 41,
        micros: 0, tzo: Some(-10800),
    };
    parse_and_assert(pdt, info, "20030925T104941-0300", None, None, false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_dayfirst0() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 10,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "10-09-2003", Some(true), None, false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_dayfirst1() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 10,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "10.09.2003", Some(true), None, false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_dayfirst2() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 10,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "10/09/2003", Some(true), None, false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_dayfirst3() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 10,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "10 09 2003", Some(true), None, false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_dayfirst4() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2007, month: 1, day: 9,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "090107", Some(true), None, false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_dayfirst5() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2015, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "2015 09 25", Some(true), None, false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_yearfirst0() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2010, month: 9, day: 3,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "10-09-03", None, Some(true), false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_yearfirst1() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2010, month: 9, day: 3,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "10.09.03", None, Some(true), false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_yearfirst2() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2010, month: 9, day: 3,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "10/09/03", None, Some(true), false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_yearfirst3() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2010, month: 9, day: 3,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "10 09 03", None, Some(true), false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_yearfirst4() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2009, month: 1, day: 7,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "090107", None, Some(true), false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_yearfirst5() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2015, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "2015 09 25", None, Some(true), false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_dfyf0() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2009, month: 7, day: 1,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "090107", Some(true), Some(true), false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_parse_dfyf1() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2015, month: 9, day: 25,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None,
    };
    parse_and_assert(pdt, info, "2015 09 25", Some(true), Some(true), false, false,
                     None, false, HashMap::new());
}

#[test]
fn test_unspecified_fallback0() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2010, 1, 31).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2009, month: 4, day: 30,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "April 2009", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_unspecified_fallback1() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2010, 1, 31).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2007, month: 2, day: 28,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Feb 2007", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_unspecified_fallback2() {
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2010, 1, 31).and_hms(0, 0, 0);
    let pdt = PyDateTime {
        year: 2008, month: 2, day: 29,
        hour: 0, minute: 0, second: 0,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Feb 2008", None, None, false, false,
                     Some(default_rsdate), false, HashMap::new());
}

#[test]
fn test_parse_ignoretz0() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 36, second: 28,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Thu Sep 25 10:36:28 BRST 2003", None, None, false, false,
                     None, true, HashMap::new());
}

#[test]
fn test_parse_ignoretz1() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 1996, month: 7, day: 10,
        hour: 15, minute: 8, second: 56,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "1996.07.10 AD at 15:08:56 PDT", None, None, false, false,
                     None, true, HashMap::new());
}

#[test]
fn test_parse_ignoretz2() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 1952, month: 4, day: 12,
        hour: 15, minute: 30, second: 42,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Tuesday, April 12, 1952 AD 3:30:42pm PST", None, None, false, false,
                     None, true, HashMap::new());
}

#[test]
fn test_parse_ignoretz3() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 1994, month: 11, day: 5,
        hour: 8, minute: 15, second: 30,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "November 5, 1994, 8:15:30 am EST", None, None, false, false,
                     None, true, HashMap::new());
}

#[test]
fn test_parse_ignoretz4() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 1994, month: 11, day: 5,
        hour: 8, minute: 15, second: 30,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "1994-11-05T08:15:30-05:00", None, None, false, false,
                     None, true, HashMap::new());
}

#[test]
fn test_parse_ignoretz5() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 1994, month: 11, day: 5,
        hour: 8, minute: 15, second: 30,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "1994-11-05T08:15:30Z", None, None, false, false,
                     None, true, HashMap::new());
}

#[test]
fn test_parse_ignoretz6() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 1976, month: 7, day: 4,
        hour: 0, minute: 1, second: 2,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "1976-07-04T00:01:02Z", None, None, false, false,
                     None, true, HashMap::new());
}

#[test]
fn test_parse_ignoretz7() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 1995, month: 4, day: 4,
        hour: 0, minute: 22, second: 12,
        micros: 0, tzo: None
    };
    parse_and_assert(pdt, info, "Tue Apr 4 00:22:12 PDT 1995", None, None, false, false,
                     None, true, HashMap::new());
}

#[test]
fn test_fuzzy0() {
    let info = ParserInfo::default();
    let pdt = PyDateTime {
        year: 2003, month: 9, day: 25,
        hour: 10, minute: 49, second: 41,
        micros: 0, tzo: Some(-10800)
    };
    parse_fuzzy_and_assert(pdt, None, info, "Today is 25 of September of 2003, exactly at 10:49:41 with timezone -03:00.", None, None, true, false,
                           None, false, HashMap::new());
}
