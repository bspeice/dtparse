extern crate chrono;

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::ParseError;
use chrono::Utc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::vec::Vec;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum Token {
    Alpha(String),
    Numeric(String),
    Separator(String),
}

pub struct Tokenizer {
    token_stack: Vec<Token>,
    parse_string: String,
}

#[derive(Debug)]
enum ParseState {
    Empty,
    Alpha,
    AlphaDecimal,
    Numeric,
    NumericDecimal,
}

impl Tokenizer {

    fn new(parse_string: String) -> Self {
        Tokenizer {
            token_stack: Vec::new(),
            parse_string: parse_string.chars().rev().collect(),
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.token_stack.is_empty() { return Some(self.token_stack.pop().unwrap()) };
        if self.parse_string.is_empty() { return None };

        let mut char_stack: Vec<char> = Vec::new();
        let mut seen_letters = false;
        let mut state = ParseState::Empty;

        while let Some(next) = self.parse_string.pop() {
            println!("{} - {:?}", next, state);
            match state {
            ParseState::Empty => {
                if next.is_numeric() {
                    state = ParseState::Numeric;
                    char_stack.push(next);
                } else if next.is_alphabetic() {
                    state = ParseState::Alpha;
                    seen_letters = true;
                    char_stack.push(next);
                } else if next.is_whitespace() {
                    char_stack.push(' ');
                    break;
                } else {
                    char_stack.push(next);
                    break;
                }
            },
            ParseState::Alpha => {
                if next.is_alphabetic() {
                    char_stack.push(next);
                } else if next == '.' {
                    state = ParseState::AlphaDecimal;
                    char_stack.push(next);
                } else {
                    // We don't recognize the character, so push it back
                    // to be handled later.
                    self.parse_string.push(next);
                    break;
                }
            },
            ParseState::AlphaDecimal => {
                if next == '.' || next.is_alphabetic() {
                    char_stack.push(next);
                } else if next.is_numeric() && char_stack.last().unwrap().clone() == '.' {
                    char_stack.push(next);
                    state = ParseState::NumericDecimal;
                } else {
                    self.parse_string.push(next);
                    break;
                }
            },
            ParseState::Numeric => {
                if next.is_numeric() {
                    char_stack.push(next);
                } else if next == '.' || (next == ',' && char_stack.len() >= 2) {
                    char_stack.push(next);
                    state = ParseState::NumericDecimal;
                } else {
                    // We don't recognize the character, so push it back
                    // to be handled later
                    self.parse_string.push(next);
                    break;
                }
            },
            ParseState::NumericDecimal => {
                if next == '.' || next.is_numeric() {
                    char_stack.push(next);
                } else if next.is_alphabetic() && char_stack.last().unwrap().clone() == '.' {
                    char_stack.push(next);
                    state = ParseState::AlphaDecimal;
                } else {
                    self.parse_string.push(next);
                    break;
                }
            }
        }};

        // I like Python's version of this much better:
        // needs_split = seen_letters or char_stack.count('.') > 1 or char_stack[-1] in '.,'
        let dot_count = char_stack.iter().fold(0, |count, character| count + (if character == &'.' {1} else {0}));
        let needs_split = seen_letters || dot_count > 1 || char_stack.last().unwrap() == &'.' || char_stack.last().unwrap() == &',';
        let final_string = char_stack.into_iter().collect();

        let mut tokens = match state {
            ParseState::Empty => vec![Token::Separator(final_string)],
            ParseState::Alpha => vec![Token::Alpha(final_string)],
            ParseState::Numeric => vec![Token::Numeric(final_string)],
            ParseState::AlphaDecimal => {
                if needs_split {
                   decimal_split(&final_string, false)
                } else {
                    vec![Token::Alpha(final_string)]
                }
            }
            ParseState::NumericDecimal => {
                if needs_split {
                    decimal_split(&final_string, dot_count == 0)
                } else {
                    vec![Token::Numeric(final_string)]
                }
            }
        }.into_iter().rev().collect();

        self.token_stack.append(&mut tokens);
        // UNWRAP: Previous match guaranteed that at least one token was added
        Some(self.token_stack.pop().unwrap())
    }
}


fn decimal_split(characters: &str, cast_period: bool) -> Vec<Token> {
    let mut token_stack: Vec<Token> = Vec::new();
    let mut char_stack: Vec<char> = Vec::new();
    let mut state = ParseState::Empty;

    for c in characters.chars() { match state {
        ParseState::Empty => {
            if c.is_alphabetic() {
                char_stack.push(c);
                state = ParseState::Alpha;
            } else if c.is_numeric() {
                char_stack.push(c);
                state = ParseState::Numeric;
            } else {
                let character = if cast_period { '.' } else { c };
                token_stack.push(Token::Separator(character.to_string()));
            }
        },
        ParseState::Alpha => {
            if c.is_alphabetic() {
                char_stack.push(c);
            } else {
                token_stack.push(Token::Alpha(char_stack.iter().collect()));
                char_stack.clear();
                let character = if cast_period { '.' } else { c };
                token_stack.push(Token::Separator(character.to_string()));
                state = ParseState::Empty;
            }
        },
        ParseState::Numeric => {
            if c.is_numeric() {
                char_stack.push(c);
            } else {
                token_stack.push(Token::Numeric(char_stack.iter().collect()));
                char_stack.clear();
                let character = if cast_period { '.' } else { c };
                token_stack.push(Token::Separator(character.to_string()));
                state = ParseState::Empty;
            }
        },
        _ => panic!("Invalid parse state during decimal_split()")
    }}

    match state {
        ParseState::Alpha => token_stack.push(Token::Alpha(char_stack.iter().collect())),
        ParseState::Numeric => token_stack.push(Token::Numeric(char_stack.iter().collect())),
        ParseState::Empty => (),
        _ => panic!("Invalid parse state during decimal_split()")
    }

    token_stack
}


pub fn tokenize(parse_string: &str) -> Vec<Token> {
    let tokenizer = Tokenizer::new(parse_string.to_owned());
    tokenizer.collect()
}


pub fn parse(date: &str) -> Result<DateTime<Utc>, ParseError> {
    let current = SystemTime::now();
    let epoch = current.duration_since(UNIX_EPOCH).unwrap();

    let naive = NaiveDateTime::from_timestamp(epoch.as_secs() as i64, epoch.subsec_nanos());

    Ok(DateTime::from_utc(naive, Utc))
}
