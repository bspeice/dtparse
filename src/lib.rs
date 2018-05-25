#![allow(dead_code)]

extern crate chrono;

use chrono::DateTime;
use chrono::Datelike;
use chrono::FixedOffset;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::Utc;
use std::collections::HashMap;
use std::vec::Vec;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidMonth,
}

enum ParseInternalError {
    // Errors that indicate internal bugs
    YMDEarlyResolve,
    YMDValueUnset,

    // Python-style errors
    ValueError(String),
}

type ParseResult<I> = Result<I, ParseError>;
type ParseIResult<I> = Result<I, ParseInternalError>;

pub struct Tokenizer {
    token_stack: Vec<String>,
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
    type Item = String;

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
        let final_string: String = char_stack.into_iter().collect();

        let mut tokens = match state {
            ParseState::Empty => vec![final_string],
            ParseState::Alpha => vec![final_string],
            ParseState::Numeric => vec![final_string],
            ParseState::AlphaDecimal => {
                if needs_split {
                    decimal_split(&final_string, false)
                } else {
                    vec![final_string]
                }
            }
            ParseState::NumericDecimal => {
                if needs_split {
                    decimal_split(&final_string, dot_count == 0)
                } else {
                    vec![final_string]
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

fn decimal_split(characters: &str, cast_period: bool) -> Vec<String> {
    let mut token_stack: Vec<String> = Vec::new();
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
                    token_stack.push(character.to_string());
                }
            }
            ParseState::Alpha => {
                if c.is_alphabetic() {
                    char_stack.push(c);
                } else {
                    token_stack.push(char_stack.iter().collect());
                    char_stack.clear();
                    let character = if cast_period { '.' } else { c };
                    token_stack.push(character.to_string());
                    state = ParseState::Empty;
                }
            }
            ParseState::Numeric => {
                if c.is_numeric() {
                    char_stack.push(c);
                } else {
                    token_stack.push(char_stack.iter().collect());
                    char_stack.clear();
                    let character = if cast_period { '.' } else { c };
                    token_stack.push(character.to_string());
                    state = ParseState::Empty;
                }
            }
            _ => panic!("Invalid parse state during decimal_split()"),
        }
    }

    match state {
        ParseState::Alpha => token_stack.push(char_stack.iter().collect()),
        ParseState::Numeric => token_stack.push(char_stack.iter().collect()),
        ParseState::Empty => (),
        _ => panic!("Invalid parse state during decimal_split()"),
    }

    token_stack
}

pub fn tokenize(parse_string: &str) -> Vec<String> {
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
            jump: parse_info(vec![
                vec![
                    " ", ".", ",", ";", "-", "/", "'", "at", "on", "and", "ad", "m", "t", "of",
                    "st", "nd", "rd", "th",
                ],
            ]),
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
            ampm: parse_info(vec![vec!["am", "a"], vec!["pm", "p"]]),
            utczone: parse_info(vec![vec!["UTC", "GMT", "Z"]]),
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

fn days_in_month(year: i32, month: i32) -> Result<i32, ParseError> {
    let leap_year = match year % 4 {
        0 => year % 400 == 0,
        _ => false,
    };

    match month {
        2 => if leap_year {
            Ok(29)
        } else {
            Ok(28)
        },
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Ok(31),
        4 | 6 | 9 | 11 => Ok(30),
        _ => Err(ParseError::InvalidMonth),
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum YMDLabel {
    Year,
    Month,
    Day,
}

struct YMD {
    _ymd: Vec<i32>, // TODO: This seems like a super weird way to store things
    century_specified: bool,
    dstridx: Option<usize>,
    mstridx: Option<usize>,
    ystridx: Option<usize>,
}

impl YMD {
    fn could_be_day(&self, val: i32) -> ParseResult<bool> {
        if self.dstridx.is_some() {
            Ok(false)
        } else if self.mstridx.is_none() {
            Ok((1 <= val) && (val <= 31))
        } else if self.ystridx.is_none() {
            // UNWRAP: mstridx guaranteed to have a value
            // TODO: Justify unwrap for self._ymd
            let month = self._ymd[self.mstridx.unwrap()];
            Ok(1 <= val && (val <= days_in_month(2000, month)?))
        } else {
            let month = self._ymd[self.mstridx.unwrap()];
            let year = self._ymd[self.ystridx.unwrap()];
            Ok(1 <= val && (val <= days_in_month(year, month)?))
        }
    }

    fn append(&mut self, val: i32, label: Option<YMDLabel>) -> ParseIResult<()> {
        let mut label = label;

        if val > 100 {
            self.century_specified = true;
            match label {
                None => label = Some(YMDLabel::Year),
                Some(YMDLabel::Year) => (),
                _ => {
                    return Err(ParseInternalError::ValueError(format!(
                        "Invalid label: {:?}",
                        label
                    )))
                }
            }
        }

        match label {
            Some(YMDLabel::Month) => {
                if self.mstridx.is_some() {
                    Err(ParseInternalError::ValueError(
                        "Month already set.".to_owned(),
                    ))
                } else {
                    self.mstridx = Some(self._ymd.len() - 1);
                    Ok(())
                }
            }
            Some(YMDLabel::Day) => {
                if self.dstridx.is_some() {
                    Err(ParseInternalError::ValueError(
                        "Day already set.".to_owned(),
                    ))
                } else {
                    self.dstridx = Some(self._ymd.len() - 1);
                    Ok(())
                }
            }
            Some(YMDLabel::Year) => {
                if self.ystridx.is_some() {
                    Err(ParseInternalError::ValueError(
                        "Year already set.".to_owned(),
                    ))
                } else {
                    self.ystridx = Some(self._ymd.len() - 1);
                    Ok(())
                }
            }
            None => Err(ParseInternalError::ValueError("Missing label.".to_owned())),
        }
    }

    fn resolve_from_stridxs(
        &mut self,
        strids: &mut HashMap<YMDLabel, usize>,
    ) -> ParseIResult<(i32, i32, i32)> {
        if strids.len() == 2 {
            let missing_key = if !strids.contains_key(&YMDLabel::Year) {
                YMDLabel::Year
            } else if !strids.contains_key(&YMDLabel::Month) {
                YMDLabel::Month
            } else {
                YMDLabel::Day
            };

            let strids_vals: Vec<usize> = strids.values().map(|u| u.clone()).collect();
            let missing_val = if !strids_vals.contains(&0) {
                0
            } else if !strids_vals.contains(&1) {
                1
            } else {
                2
            };

            strids.insert(missing_key, missing_val);
        }

        if self._ymd.len() != 3 || strids.len() != 3 {
            return Err(ParseInternalError::YMDEarlyResolve);
        }

        // TODO: Why do I have to clone &usize? Isn't it Copy?
        Ok((
            self._ymd[strids.get(&YMDLabel::Year).unwrap().clone()],
            self._ymd[strids.get(&YMDLabel::Month).unwrap().clone()],
            self._ymd[strids.get(&YMDLabel::Day).unwrap().clone()],
        ))
    }

    fn resolve_ymd(&mut self, yearfirst: bool, dayfirst: bool) -> ParseIResult<(i32, i32, i32)> {
        let len_ymd = self._ymd.len();
        let mut year: Option<i32> = None;
        let mut month: Option<i32> = None;
        let mut day: Option<i32> = None;
        let mut other: Option<i32> = None;

        let mut strids: HashMap<YMDLabel, usize> = HashMap::new();
        self.ystridx
            .map(|u| strids.insert(YMDLabel::Year, u.clone()));
        self.mstridx
            .map(|u| strids.insert(YMDLabel::Month, u.clone()));
        self.dstridx
            .map(|u| strids.insert(YMDLabel::Day, u.clone()));

        // TODO: More Rustiomatic way of doing this?
        if let Ok(ymd) = self.resolve_from_stridxs(&mut strids) {
            return Ok(ymd);
        };

        // TODO: More Rustiomatic? Too many blocks for my liking
        // Also having the array unpacking syntax is nice
        if len_ymd > 3 {
            return Err(ParseInternalError::ValueError(
                "More than three YMD values".to_owned(),
            ));
        } else if len_ymd == 1 || (self.mstridx.is_some() && len_ymd == 2) {
            if self.mstridx.is_some() {
                month = Some(self._ymd[self.mstridx.unwrap()]);
                other = Some(self._ymd[self.mstridx.unwrap() - 1]);
            } else {
                other = Some(self._ymd[0]);
            }

            if len_ymd > 1 || self.mstridx.is_some() {
                if other.unwrap_or(0) > 31 {
                    year = other;
                } else {
                    day = other;
                }
            }
        } else if len_ymd == 2 {
            if self._ymd[0] > 31 {
                year = Some(self._ymd[0]);
                month = Some(self._ymd[1]);
            } else if self._ymd[1] > 31 {
                month = Some(self._ymd[0]);
                year = Some(self._ymd[1]);
            } else if dayfirst && self._ymd[1] <= 12 {
                day = Some(self._ymd[0]);
                month = Some(self._ymd[1]);
            } else {
                month = Some(self._ymd[0]);
                day = Some(self._ymd[1]);
            }
        } else if len_ymd == 3 {
            // UNWRAP: 3 elements guarantees all indices are Some
            if self.mstridx.unwrap() == 0 {
                if self._ymd[1] > 31 {
                    month = Some(self._ymd[0]);
                    year = Some(self._ymd[1]);
                    day = Some(self._ymd[2]);
                } else {
                    month = Some(self._ymd[0]);
                    day = Some(self._ymd[1]);
                    year = Some(self._ymd[2]);
                }
            } else if self.mstridx.unwrap() == 1 {
                if self._ymd[0] > 31 || (yearfirst && self._ymd[2] <= 31) {
                    year = Some(self._ymd[0]);
                    month = Some(self._ymd[1]);
                    day = Some(self._ymd[2]);
                } else {
                    day = Some(self._ymd[0]);
                    month = Some(self._ymd[1]);
                    year = Some(self._ymd[2]);
                }
            } else if self.mstridx.unwrap() == 2 {
                // It was in the original docs, so: WTF!?
                if self._ymd[1] > 31 {
                    day = Some(self._ymd[0]);
                    year = Some(self._ymd[1]);
                    month = Some(self._ymd[2]);
                } else {
                    year = Some(self._ymd[0]);
                    day = Some(self._ymd[1]);
                    month = Some(self._ymd[2]);
                }
            } else {
                if self._ymd[0] > 31 || self.ystridx.unwrap() == 0
                    || (yearfirst && self._ymd[1] <= 12 && self._ymd[2] <= 31)
                {
                    if dayfirst && self._ymd[2] <= 12 {
                        year = Some(self._ymd[0]);
                        day = Some(self._ymd[1]);
                        month = Some(self._ymd[2]);
                    } else {
                        year = Some(self._ymd[0]);
                        month = Some(self._ymd[1]);
                        day = Some(self._ymd[2]);
                    }
                } else if self._ymd[0] > 12 || (dayfirst && self._ymd[1] <= 12) {
                    day = Some(self._ymd[0]);
                    month = Some(self._ymd[1]);
                    year = Some(self._ymd[2]);
                } else {
                    month = Some(self._ymd[0]);
                    day = Some(self._ymd[1]);
                    year = Some(self._ymd[2]);
                }
            }
        }

        // TODO: Remove the error handling here
        // We should be able to justify the UNWRAP, but I haven't
        // convinced myself of that quite yet.
        if !year.and(month).and(day).is_some() {
            Err(ParseInternalError::YMDValueUnset)
        } else {
            Ok((year.unwrap(), month.unwrap(), day.unwrap()))
        }
    }
}

struct ParsingResult {
    year: i32,
    month: i32,
    day: i32,
    weekday: bool,
    hour: i32,
    minute: i32,
    second: i32,
    microsecond: i32,
    tzname: i32,
    tzoffset: i32,
    ampm: bool,
    any_unused_tokens: Vec<String>,
}

struct Parser {
    info: ParserInfo,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            info: ParserInfo::default(),
        }
    }
}

impl Parser {
    pub fn new(info: ParserInfo) -> Self {
        Parser { info: info }
    }

    pub fn parse(
        &self,
        timestr: String,
        default: Option<NaiveDateTime>,
        ignoretz: bool,
        tzinfos: Vec<String>,
    ) -> Result<DateTime<FixedOffset>, ParseError> {
        let now = Local::now().naive_local();
        let default_date = default.unwrap_or(now).date();

        let default_ts = NaiveDateTime::new(default_date, NaiveTime::from_hms(0, 0, 0));

        // TODO: What should be done with the tokens?
        let (res, tokens) =
            self.parse_with_tokens(timestr, self.info.dayfirst, self.info.yearfirst, true, true)?;

        let naive = self.build_naive(&res, default_ts);
        Ok(self.build_tzaware(naive, &res, default_ts))
    }

    fn parse_with_tokens(
        &self,
        timestr: String,
        dayfirst: bool,
        yearfirst: bool,
        fuzzy: bool,
        fuzzy_with_tokens: bool,
    ) -> Result<(ParsingResult, Vec<String>), ParseError> {
        Err(ParseError::InvalidMonth)
    }

    fn build_naive(&self, res: &ParsingResult, default: NaiveDateTime) -> NaiveDateTime {
        Local::now().naive_local()
    }

    fn build_tzaware(
        &self,
        dt: NaiveDateTime,
        res: &ParsingResult,
        default: NaiveDateTime,
    ) -> DateTime<FixedOffset> {

        Local::now().with_timezone(&FixedOffset::east(0))
    }
}

fn parse_with_info(timestr: String, info: ParserInfo) -> Result<DateTime<FixedOffset>, ParseError> {
    let parser = Parser::new(info);
    parser.parse(timestr, None, false, vec![])
}

fn parse(timestr: String) -> Result<DateTime<FixedOffset>, ParseError> {
    parse_with_info(timestr, ParserInfo::default())
}
