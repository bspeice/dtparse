use super::Token;
use super::tokenize;

macro_rules! t {
    ($string: expr, $( $x: expr ),* ) => {
        assert_eq!(
            tokenize($string),
            vec![$( $x, )*]
        )
    };
}

macro_rules! a {
    ($string:expr) => {
        Token::Alpha($string.to_owned())
    };
}

macro_rules! n {
    ($string:expr) => {
        Token::Numeric($string.to_owned())
    };
}

macro_rules! s {
    ($string:expr) => {
        Token::Separator($string.to_owned())
    };
}

#[test]
fn test_basic_tokenize() {
    t!("Sep.2009.24",
       a!("Sep"), s!("."), n!("2009"), s!("."), n!("24"));

    t!("Sep.2009;24",
       a!("Sep"), s!("."), n!("2009"), s!(";"), n!("24"));

    t!("Sep.2009,24",
       a!("Sep"), s!("."), n!("2009"), s!(","), n!("24"));

    t!("24 Sep., 2009",
       n!("24"), s!(" "), a!("Sep"), s!("."), s!(","), s!(" "), n!("2009"));

    t!("2009.24",
       n!("2009.24"));

    t!("2009.24.09",
       n!("2009"), s!("."), n!("24"), s!("."), n!("09"));
}
