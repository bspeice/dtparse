
// WARNING
// This file was auto-generated using the `build_tests.py` script.
// Please do not edit it manually.

use tokenize;

#[test]
fn test_python_compat() {
    assert_eq!(
        tokenize("2018.5.15"),
        vec![
            "2018".to_owned(),
            ".".to_owned(),
            "5".to_owned(),
            ".".to_owned(),
            "15".to_owned(),
        ]
    );
    assert_eq!(
        tokenize("May 5, 2018"),
        vec![
            "May".to_owned(),
            " ".to_owned(),
            "5".to_owned(),
            ",".to_owned(),
            " ".to_owned(),
            "2018".to_owned(),
        ]
    );
    assert_eq!(
        tokenize("Mar. 5, 2018"),
        vec![
            "Mar".to_owned(),
            ".".to_owned(),
            " ".to_owned(),
            "5".to_owned(),
            ",".to_owned(),
            " ".to_owned(),
            "2018".to_owned(),
        ]
    );
}
