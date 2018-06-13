
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

    assert_eq!(pdt.year, rs_parsed.0.year(), "Year mismatch for {}", s);
    assert_eq!(pdt.month, rs_parsed.0.month(), "Month mismatch for {}", s);
    assert_eq!(pdt.day, rs_parsed.0.day(), "Day mismatch for {}", s);
    assert_eq!(pdt.hour, rs_parsed.0.hour(), "Hour mismatch for {}", s);
    assert_eq!(pdt.minute, rs_parsed.0.minute(), "Minute mismatch for {}", s);
    assert_eq!(pdt.second, rs_parsed.0.second(), "Second mismatch for {}", s);
    assert_eq!(pdt.micros, rs_parsed.0.timestamp_subsec_micros(), "Microsecond mismatch for {}", s);
    assert_eq!(pdt.tzo, rs_parsed.1.map(|u| u.local_minus_utc()), "Timezone Offset mismatch for {}", s);
}

macro_rules! rs_tzinfo_map {
    () => ({
        let mut h = HashMap::new();
        h.insert("BRST".to_owned(), -10800);

        h
    });
}

macro_rules! test_parse {
    ($pdt:expr, $s:expr) => {
        let info = ParserInfo::default();
        let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);

        parse_and_assert(
            $pdt,
            info,
            $s,
            None,
            None,
            false,
            false,
            Some(default_rsdate),
            false,
            rs_tzinfo_map!()
        );
    };
}

macro_rules! test_parse_yearfirst {
    ($pdt:expr, $s:expr) => {
        let info = ParserInfo::default();
        let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);

        parse_and_assert(
            $pdt,
            info,
            $s,
            None,
            Some(true),
            false,
            false,
            Some(default_rsdate),
            false,
            rs_tzinfo_map!()
        );
    };
}

macro_rules! test_parse_dayfirst {
    ($pdt:expr, $s:expr) => {
        let info = ParserInfo::default();
        let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);

        parse_and_assert(
            $pdt,
            info,
            $s,
            Some(true),
            None,
            false,
            false,
            Some(default_rsdate),
            false,
            rs_tzinfo_map!()
        );
    };
}

macro_rules! test_parse_yfdf {
    ($pdt:expr, $s:expr) => {
        let info = ParserInfo::default();
        let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);

        parse_and_assert(
            $pdt,
            info,
            $s,
            Some(true),
            None,
            true,
            true,
            Some(default_rsdate),
            false,
            rs_tzinfo_map!()
        );
    };
}

macro_rules! test_parse_ignoretz {
    ($pdt:expr, $s:expr) => {
        let info = ParserInfo::default();
        let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);

        parse_and_assert(
            $pdt,
            info,
            $s,
            None,
            None,
            false,
            false,
            Some(default_rsdate),
            true,
            rs_tzinfo_map!()
        );
    };
}

#[test]
fn test_parse0() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 10,
        minute: 36,
        second: 28,
        micros: 0,
        tzo: Some(-10800),
    };
    test_parse!(pdt, "Thu Sep 25 10:36:28 BRST 2003");
}

#[test]
fn test_parse1() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "Thu Sep 25 2003");
}

#[test]
fn test_parse2() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 10,
        minute: 49,
        second: 41,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "2003-09-25T10:49:41");
}

#[test]
fn test_parse3() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 10,
        minute: 49,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "2003-09-25T10:49");
}

#[test]
fn test_parse4() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 10,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "2003-09-25T10");
}

#[test]
fn test_parse5() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "2003-09-25");
}

#[test]
fn test_parse6() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 10,
        minute: 49,
        second: 41,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "20030925T104941");
}

#[test]
fn test_parse7() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 10,
        minute: 49,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "20030925T1049");
}

#[test]
fn test_parse8() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 10,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "20030925T10");
}

#[test]
fn test_parse9() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "20030925");
}

#[test]
fn test_parse10() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 10,
        minute: 49,
        second: 41,
        micros: 502000,
        tzo: None,
    };
    test_parse!(pdt, "2003-09-25 10:49:41,502");
}

#[test]
fn test_parse11() {
    let pdt = PyDateTime {
        year: 1997,
        month: 9,
        day: 2,
        hour: 9,
        minute: 8,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "199709020908");
}

#[test]
fn test_parse12() {
    let pdt = PyDateTime {
        year: 1997,
        month: 9,
        day: 2,
        hour: 9,
        minute: 8,
        second: 7,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "19970902090807");
}

#[test]
fn test_parse13() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "2003-09-25");
}

#[test]
fn test_parse14() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "09-25-2003");
}

#[test]
fn test_parse15() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "25-09-2003");
}

#[test]
fn test_parse16() {
    let pdt = PyDateTime {
        year: 2003,
        month: 10,
        day: 9,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "10-09-2003");
}

#[test]
fn test_parse17() {
    let pdt = PyDateTime {
        year: 2003,
        month: 10,
        day: 9,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "10-09-03");
}

#[test]
fn test_parse18() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "2003.09.25");
}

#[test]
fn test_parse19() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "09.25.2003");
}

#[test]
fn test_parse20() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "25.09.2003");
}

#[test]
fn test_parse21() {
    let pdt = PyDateTime {
        year: 2003,
        month: 10,
        day: 9,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "10.09.2003");
}

#[test]
fn test_parse22() {
    let pdt = PyDateTime {
        year: 2003,
        month: 10,
        day: 9,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "10.09.03");
}

#[test]
fn test_parse23() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "2003/09/25");
}

#[test]
fn test_parse24() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "09/25/2003");
}

#[test]
fn test_parse25() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "25/09/2003");
}

#[test]
fn test_parse26() {
    let pdt = PyDateTime {
        year: 2003,
        month: 10,
        day: 9,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "10/09/2003");
}

#[test]
fn test_parse27() {
    let pdt = PyDateTime {
        year: 2003,
        month: 10,
        day: 9,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "10/09/03");
}

#[test]
fn test_parse28() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "2003 09 25");
}

#[test]
fn test_parse29() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "09 25 2003");
}

#[test]
fn test_parse30() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "25 09 2003");
}

#[test]
fn test_parse31() {
    let pdt = PyDateTime {
        year: 2003,
        month: 10,
        day: 9,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "10 09 2003");
}

#[test]
fn test_parse32() {
    let pdt = PyDateTime {
        year: 2003,
        month: 10,
        day: 9,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "10 09 03");
}

#[test]
fn test_parse33() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "25 09 03");
}

#[test]
fn test_parse34() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "03 25 Sep");
}

#[test]
fn test_parse35() {
    let pdt = PyDateTime {
        year: 2025,
        month: 9,
        day: 3,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "25 03 Sep");
}

#[test]
fn test_parse36() {
    let pdt = PyDateTime {
        year: 1976,
        month: 7,
        day: 4,
        hour: 0,
        minute: 1,
        second: 2,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "  July   4 ,  1976   12:01:02   am  ");
}

#[test]
fn test_parse37() {
    let pdt = PyDateTime {
        year: 1996,
        month: 7,
        day: 10,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "Wed, July 10, '96");
}

#[test]
fn test_parse38() {
    let pdt = PyDateTime {
        year: 1996,
        month: 7,
        day: 10,
        hour: 12,
        minute: 8,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "1996.July.10 AD 12:08 PM");
}

#[test]
fn test_parse39() {
    let pdt = PyDateTime {
        year: 1976,
        month: 7,
        day: 4,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "July 4, 1976");
}

#[test]
fn test_parse40() {
    let pdt = PyDateTime {
        year: 1976,
        month: 7,
        day: 4,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "7 4 1976");
}

#[test]
fn test_parse41() {
    let pdt = PyDateTime {
        year: 1976,
        month: 7,
        day: 4,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "4 jul 1976");
}

#[test]
fn test_parse42() {
    let pdt = PyDateTime {
        year: 1976,
        month: 7,
        day: 4,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "7-4-76");
}

#[test]
fn test_parse43() {
    let pdt = PyDateTime {
        year: 1976,
        month: 7,
        day: 4,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "19760704");
}

#[test]
fn test_parse44() {
    let pdt = PyDateTime {
        year: 1976,
        month: 7,
        day: 4,
        hour: 0,
        minute: 1,
        second: 2,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "0:01:02 on July 4, 1976");
}

#[test]
fn test_parse45() {
    let pdt = PyDateTime {
        year: 1976,
        month: 7,
        day: 4,
        hour: 0,
        minute: 1,
        second: 2,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "0:01:02 on July 4, 1976");
}

#[test]
fn test_parse46() {
    let pdt = PyDateTime {
        year: 1976,
        month: 7,
        day: 4,
        hour: 0,
        minute: 1,
        second: 2,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "July 4, 1976 12:01:02 am");
}

#[test]
fn test_parse47() {
    let pdt = PyDateTime {
        year: 1995,
        month: 1,
        day: 2,
        hour: 4,
        minute: 24,
        second: 27,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "Mon Jan  2 04:24:27 1995");
}

#[test]
fn test_parse48() {
    let pdt = PyDateTime {
        year: 1995,
        month: 4,
        day: 4,
        hour: 0,
        minute: 22,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "04.04.95 00:22");
}

#[test]
fn test_parse49() {
    let pdt = PyDateTime {
        year: 1999,
        month: 1,
        day: 1,
        hour: 11,
        minute: 23,
        second: 34,
        micros: 578000,
        tzo: None,
    };
    test_parse!(pdt, "Jan 1 1999 11:23:34.578");
}

#[test]
fn test_parse50() {
    let pdt = PyDateTime {
        year: 1995,
        month: 4,
        day: 4,
        hour: 12,
        minute: 22,
        second: 12,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "950404 122212");
}

#[test]
fn test_parse51() {
    let pdt = PyDateTime {
        year: 2001,
        month: 5,
        day: 3,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "3rd of May 2001");
}

#[test]
fn test_parse52() {
    let pdt = PyDateTime {
        year: 2001,
        month: 3,
        day: 5,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "5th of March 2001");
}

#[test]
fn test_parse53() {
    let pdt = PyDateTime {
        year: 2003,
        month: 5,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "1st of May 2003");
}

#[test]
fn test_parse54() {
    let pdt = PyDateTime {
        year: 2008,
        month: 2,
        day: 27,
        hour: 21,
        minute: 26,
        second: 1,
        micros: 123456,
        tzo: None,
    };
    test_parse!(pdt, "20080227T21:26:01.123456789");
}

#[test]
fn test_parse55() {
    let pdt = PyDateTime {
        year: 2017,
        month: 11,
        day: 13,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "13NOV2017");
}

#[test]
fn test_parse56() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 3,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "Sep 03");
}

#[test]
fn test_parse57() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "Sep of 03");
}

#[test]
fn test_parse58() {
    let pdt = PyDateTime {
        year: 2009,
        month: 4,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "April 2009");
}

#[test]
fn test_parse59() {
    let pdt = PyDateTime {
        year: 2007,
        month: 2,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "Feb 2007");
}

#[test]
fn test_parse60() {
    let pdt = PyDateTime {
        year: 2008,
        month: 2,
        day: 25,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse!(pdt, "Feb 2008");
}

#[test]
fn test_parse_yearfirst0() {
    let pdt = PyDateTime {
        year: 2010,
        month: 9,
        day: 3,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse_yearfirst!(pdt, "10-09-03");
}

#[test]
fn test_parse_yearfirst1() {
    let pdt = PyDateTime {
        year: 2010,
        month: 9,
        day: 3,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse_yearfirst!(pdt, "10.09.03");
}

#[test]
fn test_parse_yearfirst2() {
    let pdt = PyDateTime {
        year: 2010,
        month: 9,
        day: 3,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse_yearfirst!(pdt, "10/09/03");
}

#[test]
fn test_parse_yearfirst3() {
    let pdt = PyDateTime {
        year: 2010,
        month: 9,
        day: 3,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse_yearfirst!(pdt, "10 09 03");
}

#[test]
fn test_parse_yearfirst4() {
    let pdt = PyDateTime {
        year: 2009,
        month: 1,
        day: 7,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse_yearfirst!(pdt, "090107");
}

#[test]
fn test_parse_dayfirst0() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 10,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse_dayfirst!(pdt, "10-09-2003");
}

#[test]
fn test_parse_dayfirst1() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 10,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse_dayfirst!(pdt, "10.09.2003");
}

#[test]
fn test_parse_dayfirst2() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 10,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse_dayfirst!(pdt, "10/09/2003");
}

#[test]
fn test_parse_dayfirst3() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 10,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse_dayfirst!(pdt, "10 09 2003");
}

#[test]
fn test_parse_dayfirst4() {
    let pdt = PyDateTime {
        year: 2007,
        month: 1,
        day: 9,
        hour: 0,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse_dayfirst!(pdt, "090107");
}

#[test]
fn test_parse_ignoretz0() {
    let pdt = PyDateTime {
        year: 1996,
        month: 7,
        day: 10,
        hour: 15,
        minute: 8,
        second: 56,
        micros: 0,
        tzo: None,
    };
    test_parse_ignoretz!(pdt, "1996.07.10 AD at 15:08:56 PDT");
}

#[test]
fn test_parse_ignoretz1() {
    let pdt = PyDateTime {
        year: 1952,
        month: 4,
        day: 12,
        hour: 15,
        minute: 30,
        second: 52,
        micros: 0,
        tzo: None,
    };
    test_parse_ignoretz!(pdt, "Tuesday, April 12, 1952 AD 3:30:52pm PST");
}

#[test]
fn test_parse_ignoretz2() {
    let pdt = PyDateTime {
        year: 1994,
        month: 11,
        day: 5,
        hour: 8,
        minute: 15,
        second: 30,
        micros: 0,
        tzo: None,
    };
    test_parse_ignoretz!(pdt, "November 5, 1994, 8:15:30 am EST");
}

#[test]
fn test_parse_ignoretz3() {
    let pdt = PyDateTime {
        year: 1994,
        month: 11,
        day: 5,
        hour: 8,
        minute: 15,
        second: 30,
        micros: 0,
        tzo: None,
    };
    test_parse_ignoretz!(pdt, "1994-11-05T08:15:30-05:00");
}

#[test]
fn test_parse_ignoretz4() {
    let pdt = PyDateTime {
        year: 1994,
        month: 11,
        day: 5,
        hour: 8,
        minute: 15,
        second: 30,
        micros: 0,
        tzo: None,
    };
    test_parse_ignoretz!(pdt, "1994-11-05T08:15:30Z");
}

#[test]
fn test_parse_ignoretz5() {
    let pdt = PyDateTime {
        year: 1976,
        month: 7,
        day: 4,
        hour: 0,
        minute: 1,
        second: 2,
        micros: 0,
        tzo: None,
    };
    test_parse_ignoretz!(pdt, "1976-07-04T00:01:02Z");
}

#[test]
fn test_parse_ignoretz6() {
    let pdt = PyDateTime {
        year: 1995,
        month: 4,
        day: 4,
        hour: 0,
        minute: 22,
        second: 12,
        micros: 0,
        tzo: None,
    };
    test_parse_ignoretz!(pdt, "Tue Apr 4 00:22:12 PDT 1995");
}

#[test]
fn test_parse_ignoretz7() {
    let pdt = PyDateTime {
        year: 2003,
        month: 9,
        day: 25,
        hour: 12,
        minute: 0,
        second: 0,
        micros: 0,
        tzo: None,
    };
    test_parse_ignoretz!(pdt, "0:00PM, PST");
}
