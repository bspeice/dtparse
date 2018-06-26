use ParseError;
use parse;

#[test]
fn test_fuzz() {

    assert_eq!(parse("\x2D\x38\x31\x39\x34\x38\x34"), Err(ParseError::InvalidMonth));
}