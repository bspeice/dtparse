// WARNING
// This file was auto-generated using the `build_tests.py` script.
// Please do not edit it manually.

use chrono::SecondsFormat;

use parse;

#[test]
fn test_python_compat() {
    assert_eq!(
        parse("2018.5.15".to_owned())
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Micros, false),
        "2018-05-15 04:00:00+00:00"
    );
    assert_eq!(
        parse("May 5, 2018".to_owned())
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Micros, false),
        "2018-05-05 04:00:00+00:00"
    );
    assert_eq!(
        parse("Mar. 5, 2018".to_owned())
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Micros, false),
        "2018-03-05 05:00:00+00:00"
    );
    assert_eq!(
        parse("19990101T23".to_owned())
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Micros, false),
        "1999-01-02 04:00:00+00:00"
    );
    assert_eq!(
        parse("19990101T2359".to_owned())
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Micros, false),
        "1999-01-02 04:59:00+00:00"
    );
}
