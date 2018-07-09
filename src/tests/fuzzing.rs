use chrono::NaiveDate;
use std::collections::HashMap;

use parse;
use ParseError;
use Parser;

#[test]
fn test_fuzz() {

    assert_eq!(parse("\x2D\x38\x31\x39\x34\x38\x34"), Err(ParseError::InvalidMonth));

    let default = NaiveDate::from_ymd(2016, 6, 29).and_hms(0, 0, 0);
    let mut p = Parser::default();
    let res = p.parse("\x0D\x31", None, None, false, false, Some(&default), false, HashMap::new()).unwrap();
    assert_eq!(res.0, default);

    assert_eq!(parse("\x2D\x2D\x32\x31\x38\x6D"), Err(ParseError::ImpossibleTimestamp("Invalid minute")));
}
