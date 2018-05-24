use Token;
use tokenize;

#[test]
fn test_python_compat() {
    assert_eq!(
        tokenize("2018.5.15"),
        vec![
            Token::Numeric("2018".to_owned()),
            Token::Separator(".".to_owned()),
            Token::Numeric("5".to_owned()),
            Token::Separator(".".to_owned()),
            Token::Numeric("15".to_owned()),
        ]
    );
    assert_eq!(
        tokenize("May 5, 2018"),
        vec![
            Token::Alpha("May".to_owned()),
            Token::Separator(" ".to_owned()),
            Token::Numeric("5".to_owned()),
            Token::Separator(",".to_owned()),
            Token::Separator(" ".to_owned()),
            Token::Numeric("2018".to_owned()),
        ]
    );
    assert_eq!(
        tokenize("Mar. 5, 2018"),
        vec![
            Token::Alpha("Mar".to_owned()),
            Token::Separator(".".to_owned()),
            Token::Separator(" ".to_owned()),
            Token::Numeric("5".to_owned()),
            Token::Separator(",".to_owned()),
            Token::Separator(" ".to_owned()),
            Token::Numeric("2018".to_owned()),
        ]
    );
}
