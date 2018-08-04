from dateutil.parser import parse
from datetime import datetime

tests = {
    'test_parse_default': [
        "Thu Sep 25 10:36:28",
        "Sep 10:36:28", "10:36:28", "10:36", "Sep 2003", "Sep", "2003",
        "10h36m28.5s", "10h36m28s", "10h36m", "10h", "10 h 36", "10 h 36.5",
        "36 m 5", "36 m 5 s", "36 m 05", "36 m 05 s", "10h am", "10h pm",
        "10am", "10pm", "10:00 am", "10:00 pm", "10:00am", "10:00pm",
        "10:00a.m", "10:00p.m", "10:00a.m.", "10:00p.m.",
        "October", "31-Dec-00", "0:01:02", "12h 01m02s am", "12:08 PM",
        "01h02m03", "01h02", "01h02s", "01m02", "01m02h", "2004 10 Apr 11h30m",
        # testPertain
        'Sep 03', 'Sep of 03',
        # test_hmBY - Note: This appears to be Python 3 only, no idea why
        '02:17NOV2017',
        # Weekdays
        "Thu Sep 10:36:28", "Thu 10:36:28", "Wed", "Wednesday"
    ],
    'test_parse_simple': [
        "Thu Sep 25 10:36:28 2003", "Thu Sep 25 2003", "2003-09-25T10:49:41",
        "2003-09-25T10:49", "2003-09-25T10", "2003-09-25", "20030925T104941",
        "20030925T1049", "20030925T10", "20030925", "2003-09-25 10:49:41,502",
        "199709020908", "19970902090807", "2003-09-25", "09-25-2003",
        "25-09-2003", "10-09-2003", "10-09-03", "2003.09.25", "09.25.2003",
        "25.09.2003", "10.09.2003", "10.09.03", "2003/09/25", "09/25/2003",
        "25/09/2003", "10/09/2003", "10/09/03", "2003 09 25", "09 25 2003",
        "25 09 2003", "10 09 2003", "10 09 03", "25 09 03", "03 25 Sep",
        "25 03 Sep", "  July   4 ,  1976   12:01:02   am  ",
        "Wed, July 10, '96", "1996.July.10 AD 12:08 PM", "July 4, 1976",
        "7 4 1976", "4 jul 1976", "7-4-76", "19760704",
        "0:01:02 on July 4, 1976", "0:01:02 on July 4, 1976",
        "July 4, 1976 12:01:02 am", "Mon Jan  2 04:24:27 1995",
        "04.04.95 00:22", "Jan 1 1999 11:23:34.578", "950404 122212",
        "3rd of May 2001", "5th of March 2001", "1st of May 2003",
        '0099-01-01T00:00:00', '0031-01-01T00:00:00',
        "20080227T21:26:01.123456789", '13NOV2017', '0003-03-04',
        'December.0031.30',
        # testNoYearFirstNoDayFirst
        '090107',
        # test_mstridx
        '2015-15-May',
    ],
    'test_parse_tzinfo': [
        'Thu Sep 25 10:36:28 BRST 2003', '2003 10:36:28 BRST 25 Sep Thu',
    ],
    'test_parse_offset': [
        'Thu, 25 Sep 2003 10:49:41 -0300', '2003-09-25T10:49:41.5-03:00',
        '2003-09-25T10:49:41-03:00', '20030925T104941.5-0300',
        '20030925T104941-0300'
    ],
    'test_parse_dayfirst': [
        '10-09-2003', '10.09.2003', '10/09/2003', '10 09 2003',
        # testDayFirst
        '090107',
        # testUnambiguousDayFirst
        '2015 09 25'
    ],
    'test_parse_yearfirst': [
        '10-09-03', '10.09.03', '10/09/03', '10 09 03',
        # testYearFirst
        '090107',
        # testUnambiguousYearFirst
        '2015 09 25'
    ],
    'test_parse_dfyf': [
        # testDayFirstYearFirst
        '090107',
        # testUnambiguousDayFirstYearFirst
        '2015 09 25'
    ],
    'test_unspecified_fallback': [
        'April 2009', 'Feb 2007', 'Feb 2008'
    ],
    'test_parse_ignoretz': [
        'Thu Sep 25 10:36:28 BRST 2003', '1996.07.10 AD at 15:08:56 PDT',
        'Tuesday, April 12, 1952 AD 3:30:42pm PST',
        'November 5, 1994, 8:15:30 am EST', '1994-11-05T08:15:30-05:00',
        '1994-11-05T08:15:30Z', '1976-07-04T00:01:02Z',
        'Tue Apr 4 00:22:12 PDT 1995'
    ],
    'test_fuzzy_tzinfo': [
        'Today is 25 of September of 2003, exactly at 10:49:41 with timezone -03:00.'
    ],
    'test_fuzzy_tokens_tzinfo': [
        'Today is 25 of September of 2003, exactly at 10:49:41 with timezone -03:00.'
    ],
    'test_fuzzy_simple': [
        'I have a meeting on March 1, 1974', # testFuzzyAMPMProblem
        'On June 8th, 2020, I am going to be the first man on Mars', # testFuzzyAMPMProblem
        'Meet me at the AM/PM on Sunset at 3:00 AM on December 3rd, 2003', # testFuzzyAMPMProblem
        'Meet me at 3:00 AM on December 3rd, 2003 at the AM/PM on Sunset', # testFuzzyAMPMProblem
        'Jan 29, 1945 14:45 AM I going to see you there?', # testFuzzyIgnoreAMPM
        '2017-07-17 06:15:', # test_idx_check
    ],
    'test_parse_default_ignore': [
    ],
}

def main():
    with open('src/tests/pycompat_parser.rs', 'w+') as handle:
        handle.write(TEST_HEADER)

        for test_name, test_strings in tests.items():
            for i, s in enumerate(test_strings):
                handle.write(globals()[test_name].__call__(i, s))


def test_parse_default(i, s):
    default = datetime(2003, 9, 25)
    d = parse(s, default=default)

    return TEST_PARSE_DEFAULT.format(i=i, d=d, s=s)


def test_parse_simple(i, s):
    d = parse(s)

    return TEST_PARSE_SIMPLE.format(i=i, d=d, s=s)


def test_parse_tzinfo(i, s):
    tzinfo = {'BRST': -10800}
    d = parse(s, tzinfos=tzinfo)

    return TEST_PARSE_TZINFO.format(i=i, d=d, s=s, offset=int(d.tzinfo._offset.total_seconds()))


def test_parse_offset(i, s):
    d = parse(s)
    return TEST_PARSE_OFFSET.format(i=i, d=d, s=s, offset=int(d.tzinfo._offset.total_seconds()))


def test_parse_dayfirst(i, s):
    d = parse(s, dayfirst=True)
    return TEST_PARSE_DAYFIRST.format(i=i, d=d, s=s)


def test_parse_yearfirst(i, s):
    d = parse(s, yearfirst=True)
    return TEST_PARSE_YEARFIRST.format(i=i, d=d, s=s)


def test_parse_dfyf(i, s):
    d = parse(s, dayfirst=True, yearfirst=True)
    return TEST_PARSE_DFYF.format(i=i, d=d, s=s)


def test_unspecified_fallback(i, s):
    d = parse(s, default=datetime(2010, 1, 31))
    return TEST_UNSPECIFIED_FALLBACK.format(i=i, d=d, s=s)


def test_parse_ignoretz(i, s):
    d = parse(s, ignoretz=True)
    return TEST_PARSE_IGNORETZ.format(i=i, d=d, s=s)


def test_parse_default_ignore(i, s):
    default = datetime(2003, 9, 25)
    d = parse(s, default=default)

    return TEST_PARSE_DEFAULT_IGNORE.format(i=i, d=d, s=s)


def test_fuzzy_tzinfo(i, s):
    d = parse(s, fuzzy=True)

    return TEST_FUZZY_TZINFO.format(i=i, d=d, s=s, offset=int(d.tzinfo._offset.total_seconds()))


def test_fuzzy_tokens_tzinfo(i, s):
    d, tokens = parse(s, fuzzy_with_tokens=True)

    r_tokens = ", ".join(list(map(lambda s: f'"{s}".to_owned()', tokens)))

    return TEST_FUZZY_TOKENS_TZINFO.format(
        i=i, d=d, s=s, offset=int(d.tzinfo._offset.total_seconds()),
        tokens=r_tokens
    )


def test_fuzzy_simple(i, s):
    d = parse(s, fuzzy=True)

    return TEST_FUZZY_SIMPLE.format(i=i, d=d, s=s)


# Here lies all the ugly junk.
TEST_HEADER = '''
//! This code has been generated by running the `build_pycompat.py` script
//! in the repository root. Please do not edit it, as your edits will be destroyed
//! upon re-running code generation.

extern crate chrono;

use chrono::Datelike;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::Timelike;
use std::collections::HashMap;

use Parser;
use ParserInfo;
use parse;

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
    tzinfos: &HashMap<String, i32>,
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
    assert_eq!(pdt.micros, rs_parsed.0.timestamp_subsec_micros(), "Microsecond mismatch for '{}'", s);
    assert_eq!(pdt.tzo, rs_parsed.1.map(|u| u.local_minus_utc()), "Timezone Offset mismatch for '{}'", s);
}

fn parse_and_assert_simple(
    pdt: PyDateTime,
    s: &str,
) {
    let rs_parsed = parse(s).expect(&format!("Unable to parse date in Rust '{}'", s));
    assert_eq!(pdt.year, rs_parsed.0.year(), "Year mismatch for '{}'", s);
    assert_eq!(pdt.month, rs_parsed.0.month(), "Month mismatch for '{}'", s);
    assert_eq!(pdt.day, rs_parsed.0.day(), "Day mismatch for '{}'", s);
    assert_eq!(pdt.hour, rs_parsed.0.hour(), "Hour mismatch for '{}'", s);
    assert_eq!(pdt.minute, rs_parsed.0.minute(), "Minute mismatch for '{}'", s);
    assert_eq!(pdt.second, rs_parsed.0.second(), "Second mismatch for '{}'", s);
    assert_eq!(pdt.micros, rs_parsed.0.timestamp_subsec_micros(), "Microsecond mismatch for '{}'", s);
    assert_eq!(pdt.tzo, rs_parsed.1.map(|u| u.local_minus_utc()), "Timezone Offset mismatch for '{}'", s);
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
    tzinfos: &HashMap<String, i32>,
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
    assert_eq!(pdt.micros, rs_parsed.0.timestamp_subsec_micros(), "Microsecond mismatch for '{}'", s);
    assert_eq!(pdt.tzo, rs_parsed.1.map(|u| u.local_minus_utc()), "Timezone Offset mismatch for '{}'", s);
    assert_eq!(ptokens, rs_parsed.2, "Tokens mismatch for '{}'", s);
}

macro_rules! rs_tzinfo_map {
    () => ({
        let mut h = HashMap::new();
        h.insert("BRST".to_owned(), -10800);
        h
    });
}\n'''

TEST_PARSE_DEFAULT = '''
#[test]
fn test_parse_default{i}() {{
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: None
    }};
    parse_and_assert(pdt, info, "{s}", None, None, false, false,
                     Some(default_rsdate), false, &HashMap::new());
}}\n'''

TEST_PARSE_SIMPLE = '''
#[test]
fn test_parse_simple{i}() {{
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: None,
    }};
    parse_and_assert_simple(pdt, "{s}");
}}\n'''

TEST_PARSE_TZINFO = '''
#[test]
fn test_parse_tzinfo{i}() {{
    let info = ParserInfo::default();
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: Some({offset}),
    }};
    parse_and_assert(pdt, info, "{s}", None, None, false, false,
                     None, false, &rs_tzinfo_map!());
}}\n'''

TEST_PARSE_OFFSET = '''
#[test]
fn test_parse_offset{i}() {{
    let info = ParserInfo::default();
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: Some({offset}),
    }};
    parse_and_assert(pdt, info, "{s}", None, None, false, false,
                     None, false, &HashMap::new());
}}\n'''

TEST_PARSE_DAYFIRST = '''
#[test]
fn test_parse_dayfirst{i}() {{
    let info = ParserInfo::default();
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: None,
    }};
    parse_and_assert(pdt, info, "{s}", Some(true), None, false, false,
                     None, false, &HashMap::new());
}}\n'''

TEST_PARSE_YEARFIRST = '''
#[test]
fn test_parse_yearfirst{i}() {{
    let info = ParserInfo::default();
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: None,
    }};
    parse_and_assert(pdt, info, "{s}", None, Some(true), false, false,
                     None, false, &HashMap::new());
}}\n'''

TEST_PARSE_DFYF = '''
#[test]
fn test_parse_dfyf{i}() {{
    let info = ParserInfo::default();
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: None,
    }};
    parse_and_assert(pdt, info, "{s}", Some(true), Some(true), false, false,
                     None, false, &HashMap::new());
}}\n'''

TEST_UNSPECIFIED_FALLBACK = '''
#[test]
fn test_unspecified_fallback{i}() {{
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2010, 1, 31).and_hms(0, 0, 0);
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: None
    }};
    parse_and_assert(pdt, info, "{s}", None, None, false, false,
                     Some(default_rsdate), false, &HashMap::new());
}}\n'''

TEST_PARSE_IGNORETZ = '''
#[test]
fn test_parse_ignoretz{i}() {{
    let info = ParserInfo::default();
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: None
    }};
    parse_and_assert(pdt, info, "{s}", None, None, false, false,
                     None, true, &HashMap::new());
}}\n'''

TEST_PARSE_DEFAULT_IGNORE = '''
#[test]
#[ignore]
fn test_parse_default_ignore{i}() {{
    let info = ParserInfo::default();
    let default_rsdate = &NaiveDate::from_ymd(2003, 9, 25).and_hms(0, 0, 0);
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: None
    }};
    parse_and_assert(pdt, info, "{s}", None, None, false, false,
                     Some(default_rsdate), false, &HashMap::new());
}}\n'''

TEST_FUZZY_TZINFO = '''
#[test]
fn test_fuzzy_tzinfo{i}() {{
    let info = ParserInfo::default();
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: Some({offset})
    }};
    parse_fuzzy_and_assert(pdt, None, info, "{s}", None, None, true, false,
                           None, false, &HashMap::new());
}}\n'''

TEST_FUZZY_TOKENS_TZINFO = '''
#[test]
fn test_fuzzy_tokens_tzinfo{i}() {{
    let info = ParserInfo::default();
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: Some({offset})
    }};
    let tokens = vec![{tokens}];
    parse_fuzzy_and_assert(pdt, Some(tokens), info, "{s}", None, None, true, true,
                           None, false, &HashMap::new());
}}\n'''

TEST_FUZZY_SIMPLE = '''
#[test]
fn test_fuzzy_simple{i}() {{
    let info = ParserInfo::default();
    let pdt = PyDateTime {{
        year: {d.year}, month: {d.month}, day: {d.day},
        hour: {d.hour}, minute: {d.minute}, second: {d.second},
        micros: {d.microsecond}, tzo: None
    }};
    parse_fuzzy_and_assert(pdt, None, info, "{s}", None, None, true, false,
                           None, false, &HashMap::new());
}}\n'''


if __name__ == '__main__':
    main()