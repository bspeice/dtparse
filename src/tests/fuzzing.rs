use chrono::NaiveDate;
use std::collections::HashMap;

use parse;
use ParseError;
use Parser;

#[test]
fn test_fuzz() {
    assert_eq!(
        parse("\x2D\x38\x31\x39\x34\x38\x34"),
        Err(ParseError::ImpossibleTimestamp("Invalid month"))
    );

    // Garbage in the third delimited field
    assert_eq!(
        parse("2..\x00\x000d\x00+\x010d\x01\x00\x00\x00+"),
        Err(ParseError::UnrecognizedFormat)
    );

    let default = NaiveDate::from_ymd(2016, 6, 29).and_hms(0, 0, 0);
    let p = Parser::default();
    let res = p.parse(
        "\x0D\x31",
        None,
        None,
        false,
        false,
        Some(&default),
        false,
        &HashMap::new(),
    );
    assert_eq!(res, Err(ParseError::NoDate));

    assert_eq!(
        parse("\x2D\x2D\x32\x31\x38\x6D"),
        Err(ParseError::ImpossibleTimestamp("Invalid minute"))
    );
}

#[test]
fn large_int() {
    let parse_result = parse("1412409095009.jpg");
    assert!(parse_result.is_err());
}

#[test]
fn another_large_int() {
    let parse_result = parse("1412409095009");
    assert!(parse_result.is_err());
}

#[test]
fn an_even_larger_int() {
    let parse_result = parse("1566997680962280");
    assert!(parse_result.is_err());
}

#[test]
fn empty_string() {
    assert_eq!(parse(""), Err(ParseError::NoDate))
}
