use ParseError;
use days_in_month;

#[test]
fn test_num_days_in_month() {
    assert_eq!(days_in_month(2000, 12), Ok(31));
    assert_eq!(days_in_month(2000, 2), Ok(29));
    assert_eq!(days_in_month(2000, 4), Ok(30));
    assert_eq!(days_in_month(2001, 2), Ok(28));
    assert_eq!(days_in_month(2000, 13), Err(ParseError::InvalidMonth))
}
