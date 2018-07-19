use chrono::NaiveDate;
use std::collections::HashMap;

use parse;
use ParseError;
use ParseInternalError;
use Parser;

#[test]
fn test_fuzz() {

    assert_eq!(parse("\x2D\x38\x31\x39\x34\x38\x34"), Err(ParseError::InvalidMonth));
    // Garbage in the third delimited field
    assert_eq!(parse("2..\x00\x000d\x00+\x010d\x01\x00\x00\x00+"),
               Err(ParseError::InternalError(ParseInternalError::ValueError("Unknown string format".to_owned()))));
    // OverflowError: Python int too large to convert to C long
    // assert_eq!(parse("8888884444444888444444444881"), Err(ParseError::AmPmWithoutHour));
    let default = NaiveDate::from_ymd(2016, 6, 29).and_hms(0, 0, 0);
    let mut p = Parser::default();
    let res = p.parse("\x0D\x31", None, None, false, false, Some(&default), false, HashMap::new()).unwrap();
    assert_eq!(res.0, default);

    assert_eq!(parse("\x2D\x2D\x32\x31\x38\x6D"), Err(ParseError::ImpossibleTimestamp("Invalid minute")));
}
