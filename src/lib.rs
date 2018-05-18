extern crate chrono;

use chrono::Datelike;
use chrono::Local;
use std::collections::HashMap;
use std::vec::Vec;

#[cfg(test)]
mod test_python_compat;

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
        if !self.token_stack.is_empty() {
            return Some(self.token_stack.pop().unwrap());
        };
        if self.parse_string.is_empty() {
            return None;
        };

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
                }
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
                }
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
                }
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
                }
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
            }
        }

        // I like Python's version of this much better:
        // needs_split = seen_letters or char_stack.count('.') > 1 or char_stack[-1] in '.,'
        let dot_count = char_stack.iter().fold(0, |count, character| {
            count + (if character == &'.' { 1 } else { 0 })
        });
        let needs_split = seen_letters || dot_count > 1 || char_stack.last().unwrap() == &'.'
            || char_stack.last().unwrap() == &',';
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
        }.into_iter()
            .rev()
            .collect();

        self.token_stack.append(&mut tokens);
        // UNWRAP: Previous match guaranteed that at least one token was added
        Some(self.token_stack.pop().unwrap())
    }
}

fn decimal_split(characters: &str, cast_period: bool) -> Vec<Token> {
    let mut token_stack: Vec<Token> = Vec::new();
    let mut char_stack: Vec<char> = Vec::new();
    let mut state = ParseState::Empty;

    for c in characters.chars() {
        match state {
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
            }
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
            }
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
            }
            _ => panic!("Invalid parse state during decimal_split()"),
        }
    }

    match state {
        ParseState::Alpha => token_stack.push(Token::Alpha(char_stack.iter().collect())),
        ParseState::Numeric => token_stack.push(Token::Numeric(char_stack.iter().collect())),
        ParseState::Empty => (),
        _ => panic!("Invalid parse state during decimal_split()"),
    }

    token_stack
}

pub fn tokenize(parse_string: &str) -> Vec<Token> {
    let tokenizer = Tokenizer::new(parse_string.to_owned());
    tokenizer.collect()
}

fn parse_info(vec: Vec<Vec<&str>>) -> HashMap<String, usize> {
    let mut m = HashMap::new();

    if vec.len() == 1 {
        for (i, val) in vec.get(0).unwrap().into_iter().enumerate() {
            m.insert(val.to_lowercase(), i);
        }
    } else {
        for (i, val_vec) in vec.into_iter().enumerate() {
            for val in val_vec.into_iter() {
                m.insert(val.to_lowercase(), i);
            }
        }
    }

    m
}

struct ParserInfo {
    jump: HashMap<String, usize>,
    weekday: HashMap<String, usize>,
    months: HashMap<String, usize>,
    hms: HashMap<String, usize>,
    ampm: HashMap<String, usize>,
    utczone: HashMap<String, usize>,
    pertain: HashMap<String, usize>,
    tzoffset: HashMap<String, usize>,
    dayfirst: bool,
    yearfirst: bool,
    year: u32,
    century: u32,
}


impl Default for ParserInfo {
    fn default() -> Self {
        let year = Local::now().year();
        let century = year / 100 * 100;

        ParserInfo {
            jump: parse_info(vec![vec![
                " ", ".", ",", ";", "-", "/", "'",
                "at", "on", "and", "ad", "m", "t", "of",
                "st", "nd", "rd", "th"
            ]]),
            weekday: parse_info(vec![
                vec!["Mon", "Monday"],
                vec!["Tue", "Tues", "Tuesday"],
                vec!["Wed", "Wednesday"],
                vec!["Thu", "Thurs", "Thursday"],
                vec!["Fri", "Friday"],
                vec!["Sat", "Saturday"],
                vec!["Sun", "Sunday"],
            ]),
            months: parse_info(vec![
                vec!["Jan", "January"],
                vec!["Feb", "February"],
                vec!["Mar", "March"],
                vec!["Apr", "April"],
                vec!["May"],
                vec!["Jun", "June"],
                vec!["Jul", "July"],
                vec!["Aug", "August"],
                vec!["Sep", "Sept", "September"],
                vec!["Oct", "October"],
                vec!["Nov", "November"],
                vec!["Dec", "December"],
            ]),
            hms: parse_info(vec![
                vec!["h", "hour", "hours"],
                vec!["m", "minute", "minutes"],
                vec!["s", "second", "seconds"],
            ]),
            ampm: parse_info(vec![
                vec!["am", "a"],
                vec!["pm", "p"],
            ]),
            utczone: parse_info(vec![vec![
                "UTC", "GMT", "Z"
            ]]),
            pertain: parse_info(vec![vec!["of"]]),
            tzoffset: parse_info(vec![vec![]]),
            dayfirst: false,
            yearfirst: false,
            year: year as u32,
            century: century as u32,
        }
    }
}

impl ParserInfo {
    fn get_jump(&self, name: &str) -> bool {
        self.jump.contains_key(&name.to_lowercase())
    }

    fn get_weekday(&self, name: &str) -> Option<usize> {
        self.weekday.get(&name.to_lowercase()).cloned() // TODO: Why do I have to clone a primitive?
    }

    fn get_month(&self, name: &str) -> Option<usize> {
        self.months.get(&name.to_lowercase()).map(|u| u + 1)
    }

    fn get_hms(&self, name: &str) -> Option<usize> {
        self.hms.get(&name.to_lowercase()).cloned()
    }

    fn get_ampm(&self, name: &str) -> Option<usize> {
        self.ampm.get(&name.to_lowercase()).cloned()
    }

    fn get_pertain(&self, name: &str) -> bool {
        self.pertain.contains_key(&name.to_lowercase())
    }

    fn get_utczone(&self, name: &str) -> bool {
        self.utczone.contains_key(&name.to_lowercase())
    }

    fn get_tzoffset(&self, name: &str) -> Option<usize> {
        if self.utczone.contains_key(&name.to_lowercase()) {
            Some(0)
        } else {
            self.tzoffset.get(&name.to_lowercase()).cloned()
        }
    }

    fn convertyear(&self, year: u32, century_specified: bool) -> u32 {
        let mut year = year;

        if year < 100 && !century_specified {
            year += self.century;
            if year >= self.year + 50 {
                year -= 100;
            } else if year < self.year - 50 {
                year += 100
            }
        }

        year
    }
}