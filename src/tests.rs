use chrono::Datelike;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use pyo3::FromPyObject;
use pyo3::ObjectProtocol;
use pyo3::PyDict;
use pyo3::PyList;
use pyo3::PyObject;
use pyo3::PyObjectRef;
use pyo3::Python;
use std::collections::HashMap;

use parse;
use parse_with_default;
use tokenize;

static FORMAT: &str = "%Y-%m-%d %H:%M:%S.%f";

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

macro_rules! test_parse_naive {
    // Handle tests where the times involved are unambiguous
    ($py:ident, $parser:ident, $s:expr) => {
        let dt: PyObject = $parser
            .call_method1("parse", $s)
            .unwrap()
            .extract()
            .unwrap();
        let dt_s: String = dt.call_method1($py, "strftime", FORMAT)
            .unwrap()
            .extract($py)
            .unwrap();

        let r_rs = parse($s);
        if r_rs.is_err() {
            println!("{:?}", r_rs);
            assert!(false);
        }

        let rs = r_rs.unwrap();
        assert_eq!(rs.1, None);
        // Because chrono stores nanos, and python goes to micros,
        // we have to trim a couple things off the end.
        let rs_formatted = format!("{}", rs.0.format(FORMAT));
        assert_eq!(dt_s, rs_formatted[..rs_formatted.len()-3]);
    };

    // Handle tests with some ambiguity, and thus needing a `default`
    ($py:ident, $parser:ident, $s:expr, $datetime:ident, $d:expr) => {
        let rust_date = $d.date();
        let dt_tuple = (rust_date.year(), rust_date.month(), rust_date.day());
        let pydefault: &PyObjectRef = $datetime.call_method1("datetime", dt_tuple).unwrap();

        let mut kwargs = HashMap::new();
        kwargs.insert("default", pydefault);

        let dt: PyObject = $parser
            .call_method("parse", $s, kwargs)
            .unwrap()
            .extract()
            .unwrap();
        let dt_s: String = dt.call_method1($py, "strftime", FORMAT)
            .unwrap()
            .extract($py)
            .unwrap();

        let r_rs = parse_with_default($s, $d);
        if r_rs.is_err() {
            println!("{:?}", r_rs);
            assert!(false);
        }

        let rs = r_rs.unwrap();
        assert_eq!(rs.1, None);
        // Because chrono stores nanos, and python goes to micros,
        // we have to trim a couple things off the end.
        let rs_formatted = format!("{}", rs.0.format(FORMAT));
        assert_eq!(dt_s, rs_formatted[..rs_formatted.len()-3]);
    };
}

#[test]
fn test_basic_parse() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let parser = py.import("dateutil.parser").unwrap();

    test_parse_naive!(py, parser, "2018.5.15");
    test_parse_naive!(py, parser, "May 5, 2018");
    test_parse_naive!(py, parser, "Mar. 5, 2018");
    test_parse_naive!(py, parser, "19990101T23");
    test_parse_naive!(py, parser, "19990101T2359");
}

#[test]
fn test_dateutil_compat() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let parser = py.import("dateutil.parser").unwrap();
    let datetime = py.import("datetime").unwrap();

    let default = NaiveDateTime::new(
        NaiveDate::from_ymd(2003, 9, 25),
        NaiveTime::from_hms(0, 0, 0),
    );

    // testDateCommandFormatStrip1
    test_parse_naive!(py, parser, "Thu Sep 25 10:36:28 2003", datetime, &default);
    // testDateCommandFormatStrip2
    test_parse_naive!(py, parser, "Thu Sep 25 10:36:28", datetime, &default);
    // testDateCommandFormatStrip3
    test_parse_naive!(py, parser, "Thu Sep 10:36:28", datetime, &default);
    // testDateCommandFormatStrip4
    test_parse_naive!(py, parser, "Thu 10:36:28", datetime, &default);
    // testDateCommandFormatStrip5
    test_parse_naive!(py, parser, "Sep 10:36:28", datetime, &default);
    // testDateCommandFormatStrip6
    test_parse_naive!(py, parser, "10:36:28", datetime, &default);
    // testDateCommandFormatStrip7
    test_parse_naive!(py, parser, "10:36", datetime, &default);
    // testDateCommandFormatStrip8
    test_parse_naive!(py, parser, "Thu Sep 25 2003", datetime, &default);
    // testDateCommandFormatStrip10
    test_parse_naive!(py, parser, "Sep 2003", datetime, &default);
    // testDateCommandFormatStrip11
    test_parse_naive!(py, parser, "Sep", datetime, &default);
    // testDateCommandFormatStrip12
    test_parse_naive!(py, parser, "2003", datetime, &default);

    // testISOFormatStrip2
    test_parse_naive!(py, parser, "2003-09-25T10:49:41", datetime, &default);
    // testISOFormatStrip3
    test_parse_naive!(py, parser, "2003-09-25T10:49", datetime, &default);
    // testISOFormatStrip4
    test_parse_naive!(py, parser, "2003-09-25T10", datetime, &default);
    // testISOFormatStrip5
    test_parse_naive!(py, parser, "2003-09-25", datetime, &default);

    // testISOStrippedFormatStrip2
    test_parse_naive!(py, parser, "20030925T104941", datetime, &default);
    // testISOStrippedFormatStrip3
    test_parse_naive!(py, parser, "20030925T1049", datetime, &default);
    // testISOStrippedFormatStrip4
    test_parse_naive!(py, parser, "20030925T10", datetime, &default);
    // testISOStrippedFormatStrip5
    test_parse_naive!(py, parser, "20030925", datetime, &default);

    // testPythonLoggerFormat
    test_parse_naive!(py, parser, "2003-09-25 10:49:41,502", datetime, &default);

    // testNoSeparator1
    test_parse_naive!(py, parser, "199709020908", datetime, &default);
    // testNoSeparator1
    test_parse_naive!(py, parser, "19970902090807", datetime, &default);

    // testDateWithDash1
    test_parse_naive!(py, parser, "2003-09-25", datetime, &default);
    // testDateWithDash6
    test_parse_naive!(py, parser, "09-25-2003", datetime, &default);
    // testDateWithDash7
    test_parse_naive!(py, parser, "25-09-2003", datetime, &default);
}
