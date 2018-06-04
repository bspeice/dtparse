extern crate chrono;
extern crate pyo3;

use chrono::Datelike;
use chrono::NaiveDate;
use chrono::Timelike;
use pyo3::ObjectProtocol;
use pyo3::PyBool;
use pyo3::PyDict;
use pyo3::PyList;
use pyo3::PyObject;
use pyo3::Python;
use std::collections::HashMap;

extern crate dtparse;

use dtparse::Parser;
use dtparse::ParserInfo;
use dtparse::tokenize;

macro_rules! test_split {
    ($py:ident, $timelex:ident, $s:expr) => {
        let f = $timelex.call_method1($py, "split", $s).unwrap();
        let l: &PyList = f.extract($py).unwrap();
        let s: Vec<String> = l.iter().map(|i| format!("{}", i)).collect();

        assert_eq!(s, tokenize($s));
    };
}

#[test]
fn test_split() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = py.import("dateutil.parser").unwrap();
    let t: PyObject = module.get("_timelex").unwrap().extract().unwrap();

    // TODO: Fix disagreement about whether or not to replace commas with periods
    // test_split!(py, t, "24, 50, ABC");
    test_split!(py, t, "2018.5.15");
    test_split!(py, t, "May 5, 2018");
    test_split!(py, t, "Mar. 5, 2018");
    test_split!(py, t, "19990101T23");
    test_split!(py, t, "19990101T2359");
}

macro_rules! py_offset_secs {
    ($py:ident, $dt:expr) => ({
        let float: f32 = $dt.getattr($py, "tzinfo")
            .expect("Unable to get `tzinfo` value")
            .getattr($py, "_offset")
            .expect("Unable to get `_offset` value")
            .call_method0($py, "total_seconds")
            .expect("Unable to call total_seconds()")
            .extract($py)
            .expect("Unable to extract total_seconds()");

        float as i32
    });
}

macro_rules! test_parse {
    // Full parsing options
    (
        $py:ident,
        $parser:ident,
        $datetime:ident,
        $info:expr,
        $s:expr,
        $dayfirst:expr,
        $yearfirst:expr,
        $fuzzy:expr,
        $fuzzy_with_tokens:expr,
        $default:expr,
        $ignoretz:expr,
        $tzinfos:expr
    ) => {
        let default_pydate = $datetime
            .call_method1("datetime", (2003, 9, 25))
            .expect("Unable to create default datetime");
        let default_tzinfos = PyDict::new($py);
        default_tzinfos.set_item("BRST", -10800).unwrap();

        let mut kwargs = HashMap::new();
        kwargs.insert("default", default_pydate);
        kwargs.insert("tzinfos", default_tzinfos.into());
        kwargs.insert("ignoretz", PyBool::new($py, $ignoretz).into());

        let py_true = PyBool::new($py, true);
        if $dayfirst == Some(true) {
            kwargs.insert("dayfirst", py_true.into());
        }
        if $yearfirst == Some(true) {
            kwargs.insert("yearfirst", py_true.into());
        }

        let py_parsed: PyObject = $parser
            .call_method("parse", $s, kwargs)
            .expect("Unable to call method `parse`")
            .extract()
            .expect("Unable to extract result of `parse` call");

        let mut parser = Parser::new($info);
        let rs_parsed = parser.parse(
            $s,
            $dayfirst,
            $yearfirst,
            $fuzzy,
            $fuzzy_with_tokens,
            $default,
            $ignoretz,
            $tzinfos).expect(&format!("Unable to parse date in Rust '{}'", $s));

        if let Some(tzoffset) = rs_parsed.1 {
            // Make sure the offsets are correct, and then normalize the naive date
            // to match the aware date
            let offset_secs = py_offset_secs!($py, py_parsed);

            // TODO: Should I be using utc_minus_local instead?
            assert_eq!(offset_secs, tzoffset.local_minus_utc(), "Mismatched tzoffset for '{}'", $s);
        } else {
            // First make sure that Python doesn't have any timestamps set
            let py_tzoffset = py_parsed
                .getattr($py, "tzinfo")
                .expect("Unable to get `tzinfo` value");
            
            if py_tzoffset != $py.None() {
                let offset_secs = py_offset_secs!($py, py_parsed);
                assert!(false, "Tzinfo had value {} when dtparse didn't detect timezone for '{}'", offset_secs, $s);
            }
        }

        // Naive timestamps
        let rs_dt = rs_parsed.0;

        // TODO: Should years by i32?
        let py_year: i32 = py_parsed
            .getattr($py, "year")
            .expect("Unable to get `year` value")
            .extract($py)
            .expect("Unable to convert `year` to i32");
        assert_eq!(py_year, rs_dt.year(), "Mismatched year for '{}'", $s);

        let py_month: u32 = py_parsed
            .getattr($py, "month")
            .expect("Unable to get `month` value")
            .extract($py)
            .expect("Unable to convert `month` to u32");
        assert_eq!(py_month, rs_dt.month(), "Mismatched month for '{}'", $s);

        let py_day: u32 = py_parsed
            .getattr($py, "day")
            .expect("Unable to get `day` value")
            .extract($py)
            .expect("Unable to convert `day` to u32");
        assert_eq!(py_day, rs_dt.day(), "Mismatched day for '{}'", $s);

        let py_hour: u32 = py_parsed
            .getattr($py, "hour")
            .expect("Unable to get `hour` value")
            .extract($py)
            .expect("Unable to convert `hour` to u32");
        assert_eq!(py_hour, rs_dt.hour(), "Mismatched hour for '{}'", $s);

        let py_minute: u32 = py_parsed
            .getattr($py, "minute")
            .expect("Unable to get `minute` value")
            .extract($py)
            .expect("Unable to convert `minute` to u32");
        assert_eq!(py_minute, rs_dt.minute(), "Mismatched minute for '{}'", $s);

        let py_second: u32 = py_parsed
            .getattr($py, "second")
            .expect("Unable to get `second` value")
            .extract($py)
            .expect("Unable to convert `second` to u32");
        assert_eq!(py_second, rs_dt.second(), "Mismatched second for '{}'", $s);

        let py_microsecond: u32 = py_parsed
            .getattr($py, "microsecond")
            .expect("Unable to get `microsecond` value")
            .extract($py)
            .expect("Unable to convert `microsecond` to u32");
        assert_eq!(
            py_microsecond,
            rs_dt.nanosecond() / 1000,
            "Mismatched microsecond for '{}'",
            $s
        );
    };

    (
        $py:ident,
        $parser:ident,
        $datetime:ident,
        $s:expr
    ) => {
        let info = ParserInfo::default();
        let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);

        test_parse!(
            $py,
            $parser,
            $datetime,
            info,
            $s,
            None,
            None,
            false,
            false,
            Some(default_rsdate),
            false,
            vec![]
        );
    };
}

macro_rules! test_parse_yearfirst {
    ($py:ident, $parser:ident, $datetime:ident, $s:expr) => {
        let info = ParserInfo::default();
        let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);

        test_parse!(
            $py,
            $parser,
            $datetime,
            info,
            $s,
            None,
            Some(true),
            false,
            false,
            Some(default_rsdate),
            false,
            vec![]
        );
    };
}

macro_rules! test_parse_dayfirst {
    ($py:ident, $parser:ident, $datetime:ident, $s:expr) => {
        let info = ParserInfo::default();
        let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);

        test_parse!(
            $py,
            $parser,
            $datetime,
            info,
            $s,
            Some(true),
            None,
            false,
            false,
            Some(default_rsdate),
            false,
            vec![]
        );
    };
}

macro_rules! test_parse_ignoretz {
    ($py:ident, $parser:ident, $datetime:ident, $s:expr) => {
        let info = ParserInfo::default();
        let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);

        test_parse!(
            $py,
            $parser,
            $datetime,
            info,
            $s,
            None,
            None,
            false,
            false,
            Some(default_rsdate),
            true,
            vec![]
        );
    };
}

#[test]
fn test_dateutil_compat() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let parser = py.import("dateutil.parser").unwrap();
    let datetime = py.import("datetime").unwrap();

    // TODO: Uncomment tests once timezone support is in

    // testDateCommandFormat
    // test_parse!(py, parser, datetime, "Thu Sep 25 10:36:28 BRST 2003");
    // testDateCommandFormatReversed
    // test_parse!(py, parser, datetime, "2003 10:36:28 BRST 25 Sep Thu");

    // testDateCommandFormatStrip1
    test_parse!(py, parser, datetime, "Thu Sep 25 10:36:28 2003");
    // testDateCommandFormatStrip2
    test_parse!(py, parser, datetime, "Thu Sep 25 10:36:28");
    // testDateCommandFormatStrip3
    test_parse!(py, parser, datetime, "Thu Sep 10:36:28");
    // testDateCommandFormatStrip4
    test_parse!(py, parser, datetime, "Thu 10:36:28");
    // testDateCommandFormatStrip5
    test_parse!(py, parser, datetime, "Sep 10:36:28");
    // testDateCommandFormatStrip6
    test_parse!(py, parser, datetime, "10:36:28");
    // testDateCommandFormatStrip7
    test_parse!(py, parser, datetime, "10:36");
    // testDateCommandFormatStrip8
    test_parse!(py, parser, datetime, "Thu Sep 25 2003");
    // testDateCommandFormatStrip10
    test_parse!(py, parser, datetime, "Sep 2003");
    // testDateCommandFormatStrip11
    test_parse!(py, parser, datetime, "Sep");
    // testDateCommandFormatStrip12
    test_parse!(py, parser, datetime, "2003");
    // testDateRCommandFormat
    test_parse!(py, parser, datetime, "Thu, 25 Sep 2003 10:49:41 -0300");
    // testISOFormat
    // test_parse!(py, parser, datetime, "2003-09-25T10:49:41.5-03:00");
    // TODO: tzoffset not properly recognized
    // testISOFormatStrip1
    // test_parse!(py, parser, datetime, "2003-09-25T10:49:41-03:00");
    // testISOFormatStrip2
    test_parse!(py, parser, datetime, "2003-09-25T10:49:41");
    // testISOFormatStrip3
    test_parse!(py, parser, datetime, "2003-09-25T10:49");
    // testISOFormatStrip4
    test_parse!(py, parser, datetime, "2003-09-25T10");
    // testISOFormatStrip5
    test_parse!(py, parser, datetime, "2003-09-25");
    // testISOStrippedFormat
    // test_parse!(py, parser, datetime, "20030925T104941.5-0300");
    // TODO: More than three YMD values
    // testISOStrippedFormatStrip1
    // test_parse!(py, parser, datetime, "20030925T104941-0300");
    // testISOStrippedFormatStrip2
    test_parse!(py, parser, datetime, "20030925T104941");
    // testISOStrippedFormatStrip3
    test_parse!(py, parser, datetime, "20030925T1049");
    // testISOStrippedFormatStrip4
    test_parse!(py, parser, datetime, "20030925T10");
    // testISOStrippedFormatStrip5
    test_parse!(py, parser, datetime, "20030925");
    // testPythonLoggerFormat
    test_parse!(py, parser, datetime, "2003-09-25 10:49:41,502");
    // testNoSeparator1
    test_parse!(py, parser, datetime, "199709020908");
    // testNoSeparator1
    test_parse!(py, parser, datetime, "19970902090807");
    // testDateWithDash1
    test_parse!(py, parser, datetime, "2003-09-25");
    // testDateWithDash6
    test_parse!(py, parser, datetime, "09-25-2003");
    // testDateWithDash7
    test_parse!(py, parser, datetime, "25-09-2003");
    // testDateWithDash8
    test_parse_dayfirst!(py, parser, datetime, "10-09-2003");
    // testDateWithDash9
    test_parse!(py, parser, datetime, "10-09-2003");
    // testDateWithDash10
    test_parse!(py, parser, datetime, "10-09-03");
    // testDateWithDash11
    test_parse_yearfirst!(py, parser, datetime, "10-09-03");
    // testDateWithDot1
    test_parse!(py, parser, datetime, "2003.09.25");
    // testDateWithDot6
    test_parse!(py, parser, datetime, "09.25.2003");
    // testDateWithDot7
    test_parse!(py, parser, datetime, "25.09.2003");
    // testDateWithDot8
    test_parse_dayfirst!(py, parser, datetime, "10.09.2003");
    // testDateWithDot9
    test_parse!(py, parser, datetime, "10.09.2003");
    // testDateWithDot10
    test_parse!(py, parser, datetime, "10.09.03");
    // testDateWithDot11
    test_parse_yearfirst!(py, parser, datetime, "10.09.03");
    // testDateWithSlash1
    test_parse!(py, parser, datetime, "2003/09/25");
    // testDateWithSlash6
    test_parse!(py, parser, datetime, "09/25/2003");
    // testDateWithSlash7
    test_parse!(py, parser, datetime, "25/09/2003");
    // testDateWithSlash8
    test_parse_dayfirst!(py, parser, datetime, "10/09/2003");
    // testDateWithSlash9
    test_parse!(py, parser, datetime, "10/09/2003");
    // testDateWithSlash10
    test_parse!(py, parser, datetime, "10/09/03");
    // testDateWithSlash11
    test_parse_yearfirst!(py, parser, datetime, "10/09/03");
    // testDateWithSpace1
    test_parse!(py, parser, datetime, "2003 09 25");
    // testDateWithSpace6
    test_parse!(py, parser, datetime, "09 25 2003");
    // testDateWithSpace7
    test_parse!(py, parser, datetime, "25 09 2003");
    // testDateWithSpace8
    test_parse_dayfirst!(py, parser, datetime, "10 09 2003");
    // testDateWithSpace9
    test_parse!(py, parser, datetime, "10 09 2003");
    // testDateWithSpace10
    test_parse!(py, parser, datetime, "10 09 03");
    // testDateWithSpace11
    test_parse_yearfirst!(py, parser, datetime, "10 09 03");
    // testDateWithSpace12
    test_parse!(py, parser, datetime, "25 09 03");
    // testStrangelyOrderedDate1
    test_parse!(py, parser, datetime, "03 25 Sep");
    // testStrangelyOrderedDate3
    test_parse!(py, parser, datetime, "25 03 Sep");
    // testHourWithLetters
    test_parse!(py, parser, datetime, "10h36m28.5s");
    // testHourWithLettersStrip1
    test_parse!(py, parser, datetime, "10h36m28s");
    // testHourWithLettersStrip2
    test_parse!(py, parser, datetime, "10h36m");
    // testHourWithLettersStrip3
    test_parse!(py, parser, datetime, "10h");
    // testHourWithLettersStrip4
    test_parse!(py, parser, datetime, "10 h 36");

    // TODO: Fix half a minute being 30 seconds
    // testHourWithLettersStrip5
    // test_parse!(py, parser, datetime, "10 h 36.5");

    // testMinuteWithLettersSpaces1
    test_parse!(py, parser, datetime, "36 m 5");
    // testMinuteWithLettersSpaces2
    test_parse!(py, parser, datetime, "36 m 5 s");
    // testMinuteWithLettersSpaces3
    test_parse!(py, parser, datetime, "36 m 05");
    // testMinuteWithLettersSpaces4
    test_parse!(py, parser, datetime, "36 m 05 s");

    // TODO: Add testAMPMNoHour

    // testHourAmPm1
    test_parse!(py, parser, datetime, "10h am");
    // testHourAmPm2
    test_parse!(py, parser, datetime, "10h pm");
    // testHourAmPm3
    test_parse!(py, parser, datetime, "10am");
    // testHourAmPm4
    test_parse!(py, parser, datetime, "10pm");
    // testHourAmPm5
    test_parse!(py, parser, datetime, "10:00 am");
    // testHourAmPm6
    test_parse!(py, parser, datetime, "10:00 pm");
    // testHourAmPm7
    test_parse!(py, parser, datetime, "10:00am");
    // testHourAmPm8
    test_parse!(py, parser, datetime, "10:00pm");
    // testHourAmPm9
    test_parse!(py, parser, datetime, "10:00a.m");
    // testHourAmPm10
    test_parse!(py, parser, datetime, "10:00p.m");
    // testHourAmPm11
    test_parse!(py, parser, datetime, "10:00a.m.");
    // testHourAmPm12
    test_parse!(py, parser, datetime, "10:00p.m.");

    // TODO: Add testAMPMRange

    // testPertain
    test_parse!(py, parser, datetime, "Sep 03");
    test_parse!(py, parser, datetime, "Sep of 03");

    // TODO: Handle weekdays, rather than absolute days
    // testWeekdayAlone
    // test_parse!(py, parser, datetime, "Wed");
    // testLongWeekday
    // test_parse!(py, parser, datetime, "Wednesday");

    // testLongMonth
    test_parse!(py, parser, datetime, "October");
    // testZeroYear
    test_parse!(py, parser, datetime, "31-Dec-00");

    // TODO: Handle fuzzy tests

    // testExtraSpace
    test_parse!(py, parser, datetime, "  July   4 ,  1976   12:01:02   am  ");

    // testRandomFormat1
    test_parse!(py, parser, datetime, "Wed, July 10, '96");
    // TODO: TZ support (PDT is unrecognized)
    // testRandomFormat2
    // test_parse_ignoretz!(py, parser, datetime, "1996.07.10 AD at 15:08:56 PDT");

    // testRandomFormat3
    test_parse!(py, parser, datetime, "1996.July.10 AD 12:08 PM");

    // TODO: UnrecognizedToken("PST")
    // testRandomFormat4
    // test_parse_ignoretz!(py, parser, datetime, "Tuesday, April 12, 1952 AD 3:30:42pm PST");
    // TODO: UnrecognizedToken("EST")
    // testRandomFormat5
    // test_parse_ignoretz!(py, parser, datetime, "November 5, 1994, 8:15:30 am EST");
    // TODO: Parse error - finds hour 5 instead of 8
    // testRandomFormat6
    // test_parse_ignoretz!(py, parser, datetime, "1994-11-05T08:15:30-05:00");
    // testRandomFormat7
    test_parse_ignoretz!(py, parser, datetime, "1994-11-05T08:15:30Z");
    // testRandomFormat8
    test_parse!(py, parser, datetime, "July 4, 1976");
    // testRandomFormat9
    test_parse!(py, parser, datetime, "7 4 1976");
    // testRandomFormat10
    test_parse!(py, parser, datetime, "4 jul 1976");
    // testRandomFormat11
    test_parse!(py, parser, datetime, "7-4-76");
    // testRandomFormat12
    test_parse!(py, parser, datetime, "19760704");
    // testRandomFormat13
    test_parse!(py, parser, datetime, "0:01:02");
    // testRandomFormat14
    test_parse!(py, parser, datetime, "12h 01m02s am");
    // testRandomFormat15; NB: testRandomFormat16 is exactly the same
    test_parse!(py, parser, datetime, "0:01:02 on July 4, 1976");
    // testRandomFormat17 - Needs ignoretz
    test_parse_ignoretz!(py, parser, datetime, "1976-07-04T00:01:02Z");
    // testRandomFormat18
    test_parse!(py, parser, datetime, "July 4, 1976 12:01:02 am");
    // testRandomFormat19
    test_parse!(py, parser, datetime, "Mon Jan  2 04:24:27 1995");
    // testRandomFormat20 - Needs ignoretz
    // test_parse_ignoretz!(py, parser, datetime, "Tue Apr 4 00:22:12 PDT 1995");
    // testRandomFormat21
    test_parse!(py, parser, datetime, "04.04.95 00:22");
    // testRandomFormat22
    test_parse!(py, parser, datetime, "Jan 1 1999 11:23:34.578");
    // testRandomFormat23
    test_parse!(py, parser, datetime, "950404 122212");
    // testRandomFormat24 - Needs ignoretz
    // test_parse_ignoretz!(py, parser, datetime, "0:00 PM, PST");
    // testRandomFormat25
    test_parse!(py, parser, datetime, "12:08 PM");
    // testRandomFormat26
    test_parse!(py, parser, datetime, "5:50 A.M. on June 13, 1990");
    // testRandomFormat27
    test_parse!(py, parser, datetime, "3rd of May 2001");
    // testRandomFormat28
    test_parse!(py, parser, datetime, "5th of March 2001");
    // testRandomFormat29
    test_parse!(py, parser, datetime, "1st of May 2003");
    // testRandomFormat30
    test_parse!(py, parser, datetime, "01h02m03");
    // testRandomFormat31
    test_parse!(py, parser, datetime, "01h02");
    // testRandomFormat32
    test_parse!(py, parser, datetime, "01h02s");
    // testRandomFormat33
    test_parse!(py, parser, datetime, "01m02");
    // testRandomFormat34
    test_parse!(py, parser, datetime, "01m02h");
    // testRandomFormat35
    test_parse!(py, parser, datetime, "2004 10 Apr 11h30m");
}
