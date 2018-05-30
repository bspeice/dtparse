extern crate chrono;
extern crate pyo3;

use chrono::Datelike;
use chrono::NaiveDate;
use chrono::Timelike;
use pyo3::ObjectProtocol;
use pyo3::PyDict;
use pyo3::PyList;
use pyo3::PyObject;
use pyo3::Python;
use std::collections::HashMap;

extern crate dtparse;

use dtparse::parse_with_default;
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

macro_rules! test_parse {
    ($py:ident, $parser:ident, $datetime:ident, $s:expr) => {
        println!("Attempting to parse: {}", $s);

        let default_pydate = $datetime
            .call_method1("datetime", (2003, 9, 25))
            .expect("Unable to create default datetime");
        let default_tzinfos = PyDict::new($py);
        default_tzinfos.set_item("BRST", -10800).unwrap();

        let mut kwargs = HashMap::new();
        kwargs.insert("default", default_pydate);
        kwargs.insert("tzinfos", default_tzinfos.into());

        let py_parsed: PyObject = $parser
            .call_method("parse", $s, kwargs)
            .expect("Unable to call method `parse`")
            .extract()
            .expect("Unable to extract result of `parse` call");

        let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
        let rs_parsed =
            parse_with_default($s, default_rsdate).expect("Unable to parse date in Rust");

        if let Some(_offset) = rs_parsed.1 {
            // TODO: Handle tests involving timezones
        } else {
            // Naive timestamps
            let rs_dt = rs_parsed.0;

            // First make sure that Python doesn't have any timestamps set
            let py_tzoffset = py_parsed
                .getattr($py, "tzinfo")
                .expect("Unable to get `tzinfo` value");
            assert_eq!(py_tzoffset, $py.None());

            // TODO: Should years by i32?
            let py_year: i32 = py_parsed
                .getattr($py, "year")
                .expect("Unable to get `year` value")
                .extract($py)
                .expect("Unable to convert `year` to i32");
            assert_eq!(py_year, rs_dt.year());

            let py_month: u32 = py_parsed
                .getattr($py, "month")
                .expect("Unable to get `month` value")
                .extract($py)
                .expect("Unable to convert `month` to u32");
            assert_eq!(py_month, rs_dt.month());

            let py_day: u32 = py_parsed
                .getattr($py, "day")
                .expect("Unable to get `day` value")
                .extract($py)
                .expect("Unable to convert `day` to u32");
            assert_eq!(py_day, rs_dt.day());

            let py_hour: u32 = py_parsed
                .getattr($py, "hour")
                .expect("Unable to get `hour` value")
                .extract($py)
                .expect("Unable to convert `hour` to u32");
            assert_eq!(py_hour, rs_dt.hour());

            let py_minute: u32 = py_parsed
                .getattr($py, "minute")
                .expect("Unable to get `minute` value")
                .extract($py)
                .expect("Unable to convert `minute` to u32");
            assert_eq!(py_minute, rs_dt.minute());

            let py_second: u32 = py_parsed
                .getattr($py, "second")
                .expect("Unable to get `second` value")
                .extract($py)
                .expect("Unable to convert `second` to u32");
            assert_eq!(py_second, rs_dt.second());

            let py_microsecond: u32 = py_parsed
                .getattr($py, "microsecond")
                .expect("Unable to get `microsecond` value")
                .extract($py)
                .expect("Unable to convert `microsecond` to u32");
            assert_eq!(py_microsecond, rs_dt.nanosecond() / 1000);
        }
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
    // test_parse!(py, parser, datetime, "Thu, 25 Sep 2003 10:49:41 -0300");
    // testISOFormat
    // test_parse!(py, parser, datetime, "2003-09-25T10:49:41.5-03:00");
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
    // testDateWithDash8 - Needs `dayfirst` support
    // test_parse!(py, parser, datetime, "10-09-2003");
    // testDateWithDash9
    test_parse!(py, parser, datetime, "10-09-2003");
    // testDateWithDash10
    test_parse!(py, parser, datetime, "10-09-03");
    // testDateWithDash11 - Needs `yearfirst` support
    // test_parse!(py, parser, datetime, "10-09-03")
    // testDateWithDot1
    test_parse!(py, parser, datetime, "2003.09.25");
    // testDateWithDot6
    test_parse!(py, parser, datetime, "09.25.2003");
    // testDateWithDot7
    test_parse!(py, parser, datetime, "25.09.2003");
    // testDateWithDot8 - Needs `dayfirst` support
    // test_parse!(py, parser, datetime, "10.09.2003");
    // testDateWithDot9
    test_parse!(py, parser, datetime, "10.09.2003");
    // testDateWithDot10
    test_parse!(py, parser, datetime, "10.09.03");
    // testDateWithDot11 - Needs `yearfirst` support
    // test_parse!(py, parser, datetime, "10.09.03");
    // testDateWithSlash1
    test_parse!(py, parser, datetime, "2003/09/25");
    // testDateWithSlash6
    test_parse!(py, parser, datetime, "09/25/2003");
    // testDateWithSlash7
    test_parse!(py, parser, datetime, "25/09/2003");
    // testDateWithSlash8 - Needs `dayfirst` support
    // test_parse!(py, parser, datetime, "10/09/2003");
    // testDateWithSlash9
    test_parse!(py, parser, datetime, "10/09/2003");
    // testDateWithSlash10
    test_parse!(py, parser, datetime, "10/09/03");
    // testDateWithSlash11 - Needs `yearfirst` support
    // test_parse!(py, parser, datetime, "10/09/03");
    // testDateWithSpace1
    test_parse!(py, parser, datetime, "2003 09 25");
    // testDateWithSpace6
    test_parse!(py, parser, datetime, "09 25 2003");
    // testDateWithSpace7
    test_parse!(py, parser, datetime, "25 09 2003");
    // testDateWithSpace8 - Needs `dayfirst` support
    // test_parse!(py, parser, datetime, "10 09 2003");
    // testDateWithSpace9
    test_parse!(py, parser, datetime, "10 09 2003");
    // testDateWithSpace10
    test_parse!(py, parser, datetime, "10 09 03");
    // testDateWithSpace11 - Needs `yearfirst` support
    // test_parse!(py, parser, datetime, "10 09 03");
    // testDateWithSpace12
    test_parse!(py, parser, datetime, "25 09 03");
    // testStrangelyOrderedDate1
    test_parse!(py, parser, datetime, "03 25 Sep");
    // testStrangelyOrderedDate3
    test_parse!(py, parser, datetime, "25 03 Sep");
    // TODO: Fix UnrecognizedToken error
    // testHourWithLetters
    // test_parse!(py, parser, datetime, "10h36m28.5s");
}
