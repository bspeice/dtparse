#[macro_use]
extern crate lazy_static;

extern crate chrono;
extern crate num_traits;
extern crate rust_decimal;

use chrono::Datelike;
use chrono::Duration;
use chrono::FixedOffset;
use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::Timelike;
use num_traits::cast::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal::Error as DecimalError;
use std::collections::HashMap;
use std::cmp::min;
use std::num::ParseIntError;
use std::str::FromStr;
use std::vec::Vec;

mod tokenize;
mod weekday;

#[cfg(test)]
mod tests;

use tokenize::Tokenizer;
use weekday::day_of_week;
use weekday::DayOfWeek;

lazy_static! {
    static ref ZERO: Decimal = Decimal::new(0, 0);
    static ref ONE: Decimal = Decimal::new(1, 0);
    static ref TWENTY_FOUR: Decimal = Decimal::new(24, 0);
    static ref SIXTY: Decimal = Decimal::new(60, 0);
}

#[derive(Debug, PartialEq)]
pub enum ParseInternalError {
    // Errors that indicate internal bugs
    YMDEarlyResolve,
    YMDValueUnset(Vec<YMDLabel>),
    ParseIndexError,
    InvalidDecimal,
    InvalidInteger,

    // Python-style errors
    ValueError(String),
}

impl From<DecimalError> for ParseInternalError {
    fn from(_err: DecimalError) -> Self {
        ParseInternalError::InvalidDecimal
    }
}

impl From<ParseIntError> for ParseInternalError {
    fn from(_err: ParseIntError) -> Self {
        ParseInternalError::InvalidInteger
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    AmbiguousWeekday,
    InternalError(ParseInternalError),
    InvalidMonth,
    UnrecognizedToken(String),
    InvalidParseResult(ParsingResult),
    AmPmWithoutHour,
    TimezoneUnsupported,
    ImpossibleTimestamp(&'static str),
}

impl From<ParseInternalError> for ParseError {
    fn from(err: ParseInternalError) -> Self {
        ParseError::InternalError(err)
    }
}

type ParseResult<I> = Result<I, ParseError>;
type ParseIResult<I> = Result<I, ParseInternalError>;

pub fn tokenize(parse_string: &str) -> Vec<String> {
    let tokenizer = Tokenizer::new(parse_string);
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

#[derive(Debug, PartialEq)]
pub struct ParserInfo {
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
    year: i32,
    century: i32,
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
            year: year,
            century: century,
        }
    }
}

impl ParserInfo {
    fn get_jump(&self, name: &str) -> bool {
        self.jump.contains_key(&name.to_lowercase())
    }

    fn get_weekday(&self, name: &str) -> Option<usize> {
        self.weekday.get(&name.to_lowercase()).map(|i| *i)
    }

    fn get_month(&self, name: &str) -> Option<usize> {
        self.months.get(&name.to_lowercase()).map(|u| u + 1)
    }

    fn get_hms(&self, name: &str) -> Option<usize> {
        self.hms.get(&name.to_lowercase()).map(|i| *i)
    }

    fn get_ampm(&self, name: &str) -> Option<bool> {
        if let Some(v) = self.ampm.get(&name.to_lowercase()) {
            // Python technically uses numbers here, but given that the numbers are
            // only 0 and 1, it's easier to use booleans
            Some(*v == 1)
        } else {
            None
        }
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

    fn convertyear(&self, year: i32, century_specified: bool) -> i32 {
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

    // TODO: Should this be moved elsewhere?
    fn validate(&self, res: &mut ParsingResult) -> bool {
        if let Some(y) = res.year {
            res.year = Some(self.convertyear(y, res.century_specified))
        };

        if res.tzoffset == Some(0) && res.tzname.is_none() || res.tzname == Some("Z".to_owned()) {
            res.tzname = Some("UTC".to_owned());
            res.tzoffset = Some(0);
        } else if res.tzoffset != Some(0) && res.tzname.is_some()
            && self.get_utczone(res.tzname.as_ref().unwrap())
        {
            res.tzoffset = Some(0);
        }

        true
    }
}

fn days_in_month(year: i32, month: i32) -> Result<u32, ParseError> {
    let leap_year = match year % 4 {
        0 => year % 400 != 0,
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
        _ => {
            Err(ParseError::InvalidMonth)
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum YMDLabel {
    Year,
    Month,
    Day,
}

#[derive(Debug, Default)]
struct YMD {
    _ymd: Vec<i32>, // TODO: This seems like a super weird way to store things
    century_specified: bool,
    dstridx: Option<usize>,
    mstridx: Option<usize>,
    ystridx: Option<usize>,
}

impl YMD {
    fn len(&self) -> usize {
        self._ymd.len()
    }

    fn could_be_day(&self, val: i32) -> bool {
        if self.dstridx.is_some() {
            false
        } else if self.mstridx.is_none() {
            (1 <= val) && (val <= 31)
        } else if self.ystridx.is_none() {
            // UNWRAP: Earlier condition catches mstridx missing
            let month = self._ymd[self.mstridx.unwrap()];
            1 <= val && (val <= days_in_month(2000, month).unwrap() as i32)
        } else {
            // UNWRAP: Earlier conditions prevent us from unsafely unwrapping
            let month = self._ymd[self.mstridx.unwrap()];
            let year = self._ymd[self.ystridx.unwrap()];
            1 <= val && (val <= days_in_month(year, month).unwrap() as i32)
        }
    }

    fn append(&mut self, val: i32, token: &str, label: Option<YMDLabel>) -> ParseIResult<()> {
        let mut label = label;

        // Python auto-detects strings using the '__len__' function here.
        // We instead take in both and handle as necessary.
        if Decimal::from_str(token).is_ok() && token.len() > 2 {
            self.century_specified = true;
            match label {
                None | Some(YMDLabel::Year) => label = Some(YMDLabel::Year),
                _ => {
                    return Err(ParseInternalError::ValueError(format!(
                        "Invalid label {:?} for token {:?}",
                        label,
                        token
                    )))
                }
            }
        }

        if val > 100 {
            self.century_specified = true;
            match label {
                None => label = Some(YMDLabel::Year),
                Some(YMDLabel::Year) => (),
                _ => {
                    return Err(ParseInternalError::ValueError(format!(
                        "Invalid label {:?} for token {:?}",
                        label,
                        token
                    )))
                }
            }
        }

        self._ymd.push(val);

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
            None => Ok(()),
        }
    }

    fn resolve_from_stridxs(
        &mut self,
        strids: &mut HashMap<YMDLabel, usize>,
    ) -> ParseIResult<(Option<i32>, Option<i32>, Option<i32>)> {
        if self._ymd.len() == 3 && strids.len() == 2 {
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

        if self._ymd.len() != strids.len() {
            return Err(ParseInternalError::YMDEarlyResolve);
        }

        Ok((
            strids
                .get(&YMDLabel::Year)
                .map(|i| self._ymd[*i]),
            strids
                .get(&YMDLabel::Month)
                .map(|i| self._ymd[*i]),
            strids
                .get(&YMDLabel::Day)
                .map(|i| self._ymd[*i]),
        ))
    }

    fn resolve_ymd(
        &mut self,
        yearfirst: bool,
        dayfirst: bool,
    ) -> ParseIResult<(Option<i32>, Option<i32>, Option<i32>)> {
        let len_ymd = self._ymd.len();

        let mut strids: HashMap<YMDLabel, usize> = HashMap::new();
        self.ystridx
            .map(|u| strids.insert(YMDLabel::Year, u.clone()));
        self.mstridx
            .map(|u| strids.insert(YMDLabel::Month, u.clone()));
        self.dstridx
            .map(|u| strids.insert(YMDLabel::Day, u.clone()));

        // TODO: More Rustiomatic way of doing this?
        if len_ymd == strids.len() && strids.len() > 0
            || (len_ymd == 3 && strids.len() == 2)
        {
            return self.resolve_from_stridxs(&mut strids);
        };

        if len_ymd > 3 {
            return Err(ParseInternalError::ValueError(
                "More than three YMD values".to_owned(),
            ));
        }

        match (len_ymd, self.mstridx) {
            (1, Some(val)) |
            (2, Some(val)) => {
                let other = if len_ymd == 1 {
                    self._ymd[0]
                } else {
                    self._ymd[1 - val]
                };
                if other > 31 {
                    return Ok((Some(other), Some(self._ymd[val]), None));
                }
                return Ok((None, Some(self._ymd[val]), Some(other)));
            },
            (2, None) => {
                if self._ymd[0] > 31 {
                    return Ok((Some(self._ymd[0]), Some(self._ymd[1]), None));
                }
                if self._ymd[1] > 31 {
                    return Ok((Some(self._ymd[1]), Some(self._ymd[0]), None));
                }
                if dayfirst && self._ymd[1] <= 12 {
                    return Ok((None, Some(self._ymd[1]), Some(self._ymd[0])));
                }
                return Ok((None, Some(self._ymd[0]), Some(self._ymd[1])));
            },
            (3, Some(0)) => {
                if self._ymd[1] > 31 {
                    return Ok((Some(self._ymd[1]), Some(self._ymd[0]), Some(self._ymd[2])));
                }
                return Ok((Some(self._ymd[2]), Some(self._ymd[0]), Some(self._ymd[1])));
            },
            (3, Some(1)) => {
                if self._ymd[0] > 31 || (yearfirst && self._ymd[2] <= 31) {
                    return Ok((Some(self._ymd[0]), Some(self._ymd[1]), Some(self._ymd[2])));
                }
                return Ok((Some(self._ymd[2]), Some(self._ymd[1]), Some(self._ymd[0])));
            },
            (3, Some(2)) => {
                // It was in the original docs, so: WTF!?
                if self._ymd[1] > 31 {
                    return Ok((Some(self._ymd[2]), Some(self._ymd[1]), Some(self._ymd[0])));
                }
                return Ok((Some(self._ymd[0]), Some(self._ymd[2]), Some(self._ymd[1])));
            },
            (3, None) => {
                if self._ymd[0] > 31 || self.ystridx == Some(0)
                    || (yearfirst && self._ymd[1] <= 12 && self._ymd[2] <= 31)
                {
                    if dayfirst && self._ymd[2] <= 12 {
                        return Ok((Some(self._ymd[0]), Some(self._ymd[2]), Some(self._ymd[1])));
                    }
                    return Ok((Some(self._ymd[0]), Some(self._ymd[1]), Some(self._ymd[2])));
                } else if self._ymd[0] > 12 || (dayfirst && self._ymd[1] <= 12) {
                    return Ok((Some(self._ymd[2]), Some(self._ymd[1]), Some(self._ymd[0])));
                }
                return Ok((Some(self._ymd[2]), Some(self._ymd[0]), Some(self._ymd[1])));
            },
            (_, _) => { return Ok((None, None, None)); },
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ParsingResult {
    year: Option<i32>,
    month: Option<i32>,
    day: Option<i32>,
    weekday: Option<usize>,
    hour: Option<i32>,
    minute: Option<i32>,
    second: Option<i32>,
    microsecond: Option<i32>,
    tzname: Option<String>,
    tzoffset: Option<i32>,
    ampm: Option<bool>,
    century_specified: bool,
    any_unused_tokens: Vec<String>,
}

#[derive(Default)]
pub struct Parser {
    info: ParserInfo,
}

impl Parser {
    pub fn new(info: ParserInfo) -> Self {
        Parser { info }
    }

    pub fn parse(
        &mut self,
        timestr: &str,
        dayfirst: Option<bool>,
        yearfirst: Option<bool>,
        fuzzy: bool,
        fuzzy_with_tokens: bool,
        default: Option<&NaiveDateTime>,
        ignoretz: bool,
        tzinfos: HashMap<String, i32>,
    ) -> ParseResult<(NaiveDateTime, Option<FixedOffset>, Option<Vec<String>>)> {
        let default_date = default.unwrap_or(&Local::now().naive_local()).date();

        let default_ts = NaiveDateTime::new(default_date, NaiveTime::from_hms(0, 0, 0));

        let (res, tokens) =
            self.parse_with_tokens(timestr, dayfirst, yearfirst, fuzzy, fuzzy_with_tokens)?;

        let naive = self.build_naive(&res, &default_ts)?;

        if !ignoretz {
            let offset = self.build_tzaware(&naive, &res, tzinfos)?;
            Ok((naive, offset, tokens))
        } else {
            Ok((naive, None, tokens))
        }
    }

    fn parse_with_tokens(
        &mut self,
        timestr: &str,
        dayfirst: Option<bool>,
        yearfirst: Option<bool>,
        fuzzy: bool,
        fuzzy_with_tokens: bool,
    ) -> Result<(ParsingResult, Option<Vec<String>>), ParseError> {
        let fuzzy = if fuzzy_with_tokens { true } else { fuzzy };
        // This is probably a stylistic abomination
        let dayfirst = if let Some(dayfirst) = dayfirst {
            dayfirst
        } else {
            self.info.dayfirst
        };
        let yearfirst = if let Some(yearfirst) = yearfirst {
            yearfirst
        } else {
            self.info.yearfirst
        };

        let mut res = ParsingResult::default();

        let mut l = tokenize(&timestr);
        let mut skipped_idxs: Vec<usize> = Vec::new();

        let mut ymd = YMD::default();

        let len_l = l.len();
        let mut i = 0;

        while i < len_l {
            let value_repr = l[i].clone();

            if let Ok(_v) = Decimal::from_str(&value_repr) {
                i = self.parse_numeric_token(&l, i, &self.info, &mut ymd, &mut res, fuzzy)?;
            } else if let Some(value) = self.info.get_weekday(&l[i]) {
                res.weekday = Some(value);
            } else if let Some(value) = self.info.get_month(&l[i]) {
                ymd.append(value as i32, &l[i], Some(YMDLabel::Month))?;

                if i + 1 < len_l {
                    if l[i + 1] == "-" || l[i + 1] == "/" {
                        // Jan-01[-99]
                        let sep = &l[i + 1];
                        // TODO: This seems like a very unsafe unwrap
                        ymd.append(l[i + 2].parse::<i32>().unwrap(), &l[i + 2], None)?;

                        if i + 3 < len_l && &l[i + 3] == sep {
                            // Jan-01-99
                            ymd.append(l[i + 4].parse::<i32>().unwrap(), &l[i + 4], None)?;
                            i += 2;
                        }

                        i += 2;
                    } else if i + 4 < len_l && l[i + 1] == l[i + 3] && l[i + 3] == " "
                        && self.info.get_pertain(&l[i + 2])
                    {
                        // Jan of 01
                        if let Some(value) = l[i + 4].parse::<i32>().ok() {
                            let year = self.info.convertyear(value, false);
                            ymd.append(year, &l[i + 4], Some(YMDLabel::Year))?;
                        }

                        i += 4;
                    }
                }
            } else if let Some(value) = self.info.get_ampm(&l[i]) {
                let is_ampm = self.ampm_valid(res.hour, res.ampm, fuzzy);

                if is_ampm == Ok(true) {
                    res.hour = res.hour.map(|h| self.adjust_ampm(h, value));
                    res.ampm = Some(value);
                } else if fuzzy {
                    skipped_idxs.push(i);
                }
            } else if self.could_be_tzname(res.hour, res.tzname.clone(), res.tzoffset, &l[i]) {
                res.tzname = Some(l[i].clone());

                let tzname = res.tzname.clone().unwrap();
                res.tzoffset = self.info.get_tzoffset(&tzname).map(|t| t as i32);

                if i + 1 < len_l && (l[i + 1] == "+" || l[i + 1] == "-") {
                    // GMT+3
                    // According to dateutil docs - reverse the size, as GMT+3 means
                    // "my time +3 is GMT" not "GMT +3 is my time"

                    // TODO: Is there a better way of in-place modifying a vector?
                    let item = if l[i + 1] == "+" {
                        "-".to_owned()
                    } else {
                        "-".to_owned()
                    };
                    l.remove(i + 1);
                    l.insert(i + 1, item);

                    res.tzoffset = None;

                    if self.info.get_utczone(&tzname) {
                        res.tzname = None;
                    }
                }
            } else if res.hour.is_some() && (l[i] == "+" || l[i] == "-") {
                let signal = if l[i] == "+" { 1 } else { -1 };
                let len_li = l[i].len();

                let mut hour_offset: Option<i32> = None;
                let mut min_offset: Option<i32> = None;

                // TODO: check that l[i + 1] is integer?
                if len_li == 4 {
                    // -0300
                    hour_offset = Some(l[i + 1][..2].parse::<i32>().unwrap());
                    min_offset = Some(l[i + 1][2..4].parse::<i32>().unwrap());
                } else if i + 2 < len_l && l[i + 2] == ":" {
                    // -03:00
                    hour_offset = Some(l[i + 1].parse::<i32>().unwrap());
                    min_offset = Some(l[i + 3].parse::<i32>().unwrap());
                    i += 2;
                } else if len_li <= 2 {
                    // -[0]3
                    hour_offset = Some(l[i + 1][..2].parse::<i32>().unwrap());
                    min_offset = Some(0);
                }

                res.tzoffset =
                    Some(signal * (hour_offset.unwrap() * 3600 + min_offset.unwrap() * 60));

                let tzname = res.tzname.clone();
                if i + 5 < len_l && self.info.get_jump(&l[i + 2]) && l[i + 3] == "("
                    && l[i + 5] == ")" && 3 <= l[i + 4].len()
                    && self.could_be_tzname(res.hour, tzname, None, &l[i + 4])
                {
                    // (GMT)
                    res.tzname = Some(l[i + 4].clone());
                    i += 4;
                }

                i += 1;
            } else if !(self.info.get_jump(&l[i]) || fuzzy) {
                return Err(ParseError::UnrecognizedToken(l[i].clone()));
            } else {
                skipped_idxs.push(i);
            }

            i += 1;
        }

        let (year, month, day) = ymd.resolve_ymd(yearfirst, dayfirst)?;

        res.century_specified = ymd.century_specified;
        res.year = year;
        res.month = month;
        res.day = day;

        if !self.info.validate(&mut res) {
            Err(ParseError::InvalidParseResult(res))
        } else if fuzzy_with_tokens {
            let skipped_tokens = self.recombine_skipped(skipped_idxs, l);
            Ok((res, Some(skipped_tokens)))
        } else {
            Ok((res, None))
        }
    }

    fn could_be_tzname(
        &self,
        hour: Option<i32>,
        tzname: Option<String>,
        tzoffset: Option<i32>,
        token: &str,
    ) -> bool {
        let all_ascii_upper = token
            .chars()
            .all(|c| 65u8 as char <= c && c <= 90u8 as char);
        return hour.is_some() && tzname.is_none() && tzoffset.is_none() && token.len() <= 5
            && all_ascii_upper;
    }

    fn ampm_valid(&self, hour: Option<i32>, ampm: Option<bool>, fuzzy: bool) -> ParseResult<bool> {
        let mut val_is_ampm = true;

        if fuzzy && ampm.is_some() {
            val_is_ampm = false;
        }

        if hour.is_none() {
            if fuzzy {
                val_is_ampm = false;
            } else {
                return Err(ParseError::AmPmWithoutHour);
            }
        } else if !(0 <= hour.unwrap() && hour.unwrap() <= 12) {
            if fuzzy {
                val_is_ampm = false;
            } else {
                return Err(ParseError::ImpossibleTimestamp("Invalid hour"));
            }
        }

        Ok(val_is_ampm)
    }

    fn build_naive(&self, res: &ParsingResult, default: &NaiveDateTime) -> ParseResult<NaiveDateTime> {
        let y = res.year.unwrap_or(default.year());
        let m = res.month.unwrap_or(default.month() as i32) as u32;

        let d_offset = if res.weekday.is_some() && res.day.is_none() {
            // TODO: Unwrap not justified
            let dow = day_of_week(y as u32, m, default.day()).unwrap();

            // UNWRAP: We've already check res.weekday() is some
            let actual_weekday = (res.weekday.unwrap() + 1) % 7;
            let other = DayOfWeek::from_numeral(actual_weekday as u32);
            Duration::days(dow.difference(other) as i64)
        } else {
            Duration::days(0)
        };

        // TODO: Change month/day to u32
        let d = NaiveDate::from_ymd(
            y,
            m,
            min(res.day.unwrap_or(default.day() as i32) as u32, days_in_month(y, m as i32)?)
        );

        let d = d + d_offset;

        let hour = res.hour.unwrap_or(default.hour() as i32) as u32;
        let minute = res.minute.unwrap_or(default.minute() as i32) as u32;
        let second = res.second.unwrap_or(default.second() as i32) as u32;
        let microsecond = res.microsecond
            .unwrap_or(default.timestamp_subsec_micros() as i32) as u32;
        let t = NaiveTime::from_hms_micro_opt(hour, minute, second, microsecond).ok_or_else(|| {
            if hour >= 24 {
                ParseError::ImpossibleTimestamp("Invalid hour")
            } else if minute >= 60 {
                ParseError::ImpossibleTimestamp("Invalid minute")
            } else if second >= 60 {
                ParseError::ImpossibleTimestamp("Invalid second")
            } else if microsecond >= 2_000_000 {
                ParseError::ImpossibleTimestamp("Invalid microsecond")
            } else {
                unreachable!();
            }
        })?;

        Ok(NaiveDateTime::new(d, t))
    }

    fn build_tzaware(
        &self,
        _dt: &NaiveDateTime,
        res: &ParsingResult,
        tzinfos: HashMap<String, i32>,
    ) -> ParseResult<Option<FixedOffset>> {
        // TODO: Actual timezone support
        if let Some(offset) = res.tzoffset {
            Ok(Some(FixedOffset::east(offset)))
        } else if res.tzoffset == None
            && (res.tzname == Some(" ".to_owned()) || res.tzname == Some(".".to_owned())
                || res.tzname == Some("-".to_owned()) || res.tzname == None)
        {
            Ok(None)
        } else if res.tzname.is_some() && tzinfos.contains_key(res.tzname.as_ref().unwrap()) {
            Ok(Some(FixedOffset::east(
                tzinfos.get(res.tzname.as_ref().unwrap()).unwrap().clone(),
            )))
        } else if res.tzname.is_some() {
            // TODO: Dateutil issues a warning/deprecation notice here. Should we force the issue?
            println!("tzname {} identified but not understood. Ignoring for the time being, but behavior is subject to change.", res.tzname.as_ref().unwrap());
            Ok(None)
        } else {
            Err(ParseError::TimezoneUnsupported)
        }
    }

    fn parse_numeric_token(
        &self,
        tokens: &Vec<String>,
        idx: usize,
        info: &ParserInfo,
        ymd: &mut YMD,
        res: &mut ParsingResult,
        fuzzy: bool,
    ) -> Result<usize, ParseInternalError> {
        let mut idx = idx;
        let value_repr = &tokens[idx];
        let mut value = Decimal::from_str(&value_repr).unwrap();

        let len_li = value_repr.len();
        let len_l = tokens.len();

        // TODO: I miss the `x in y` syntax
        // TODO: Decompose this logic a bit
        if ymd.len() == 3 && (len_li == 2 || len_li == 4) && res.hour.is_none()
            && (idx + 1 >= len_l
                || (tokens[idx + 1] != ":" && info.get_hms(&tokens[idx + 1]).is_none()))
        {
            // 1990101T32[59]
            let s = &tokens[idx];
            res.hour = s[0..2].parse::<i32>().ok();

            if len_li == 4 {
                res.minute = Some(s[2..4].parse::<i32>()?)
            }
        } else if len_li == 6 || (len_li > 6 && tokens[idx].find(".") == Some(6)) {
            // YYMMDD or HHMMSS[.ss]
            let s = &tokens[idx];

            if ymd.len() == 0 && tokens[idx].find(".") == None {
                ymd.append(s[0..2].parse::<i32>().unwrap(), &s[0..2], None)?;
                ymd.append(s[2..4].parse::<i32>().unwrap(), &s[2..4], None)?;
                ymd.append(s[4..6].parse::<i32>().unwrap(), &s[4..6], None)?;
            } else {
                // 19990101T235959[.59]
                res.hour = s[0..2].parse::<i32>().ok();
                res.minute = s[2..4].parse::<i32>().ok();

                let t = self.parsems(&s[4..])?;
                res.second = Some(t.0);
                res.microsecond = Some(t.1);
            }
        } else if vec![8, 12, 14].contains(&len_li) {
            // YYMMDD
            let s = &tokens[idx];
            ymd.append(s[..4].parse::<i32>().unwrap(), &s[..4], Some(YMDLabel::Year))?;
            ymd.append(s[4..6].parse::<i32>().unwrap(), &s[4..6], None)?;
            ymd.append(s[6..8].parse::<i32>().unwrap(), &s[6..8], None)?;

            if len_li > 8 {
                res.hour = Some(s[8..10].parse::<i32>()?);
                res.minute = Some(s[10..12].parse::<i32>()?);

                if len_li > 12 {
                    res.second = Some(s[12..].parse::<i32>()?);
                }
            }
        } else if let Some(hms_idx) = self.find_hms_index(idx, tokens, info, true) {
            // HH[ ]h or MM[ ]m or SS[.ss][ ]s
            let (new_idx, hms) = self.parse_hms(idx, tokens, info, Some(hms_idx));
            if hms.is_some() {
                // TODO: This unwrap is unjustified.
                self.assign_hms(res, value_repr, hms.unwrap());
            }
            idx = new_idx;
        } else if idx + 2 < len_l && tokens[idx + 1] == ":" {
            // HH:MM[:SS[.ss]]
            // TODO: Better story around Decimal handling
            res.hour = Some(value.floor().to_i64().unwrap() as i32);
            // TODO: Rescope `value` here?
            value = self.to_decimal(&tokens[idx + 2]);
            let min_sec = self.parse_min_sec(value);
            res.minute = Some(min_sec.0);
            res.second = min_sec.1;

            if idx + 4 < len_l && tokens[idx + 3] == ":" {
                // TODO: (x, y) = (a, b) syntax?
                let ms = self.parsems(&tokens[idx + 4]).unwrap();
                res.second = Some(ms.0);
                res.microsecond = Some(ms.1);

                idx += 2;
            }
            idx += 2;
        } else if idx + 1 < len_l
            && (tokens[idx + 1] == "-" || tokens[idx + 1] == "/" || tokens[idx + 1] == ".")
        {
            // TODO: There's got to be a better way of handling the condition above
            let sep = &tokens[idx + 1];
            ymd.append(value_repr.parse::<i32>().unwrap(), &value_repr, None)?;

            if idx + 2 < len_l && !info.get_jump(&tokens[idx + 2]) {
                if let Ok(val) = tokens[idx + 2].parse::<i32>() {
                    ymd.append(val, &tokens[idx + 2], None)?;
                } else if let Some(val) = info.get_month(&tokens[idx + 2]) {
                    ymd.append(val as i32, &tokens[idx + 2], Some(YMDLabel::Month))?;
                }

                if idx + 3 < len_l && &tokens[idx + 3] == sep {
                    if let Some(value) = info.get_month(&tokens[idx + 4]) {
                        ymd.append(value as i32, &tokens[idx + 4], Some(YMDLabel::Month))?;
                    } else {
                        ymd.append(tokens[idx + 4].parse::<i32>().unwrap(), &tokens[idx + 4], None)?;
                    }

                    idx += 2;
                }

                idx += 1;
            }

            idx += 1
        } else if idx + 1 >= len_l || info.get_jump(&tokens[idx + 1]) {
            if idx + 2 < len_l && info.get_ampm(&tokens[idx + 2]).is_some() {
                let hour = value.to_i64().unwrap() as i32;
                let ampm = info.get_ampm(&tokens[idx + 2]).unwrap();
                res.hour = Some(self.adjust_ampm(hour, ampm));
                idx += 1;
            } else {
                ymd.append(value.floor().to_i64().unwrap() as i32, &value_repr, None)?;
            }

            idx += 1;
        } else if info.get_ampm(&tokens[idx + 1]).is_some()
            && (*ZERO <= value && value < *TWENTY_FOUR)
        {
            // 12am
            let hour = value.to_i64().unwrap() as i32;
            res.hour = Some(self.adjust_ampm(hour, info.get_ampm(&tokens[idx + 1]).unwrap()));
            idx += 1;
        } else if ymd.could_be_day(value.to_i64().unwrap() as i32) {
            ymd.append(value.to_i64().unwrap() as i32, &value_repr, None)?;
        } else if !fuzzy {
            return Err(ParseInternalError::ValueError("".to_owned()));
        }

        Ok(idx)
    }

    fn adjust_ampm(&self, hour: i32, ampm: bool) -> i32 {
        if hour < 12 && ampm {
            hour + 12
        } else if hour == 12 && !ampm {
            0
        } else {
            hour
        }
    }

    fn parsems(&self, seconds_str: &str) -> Result<(i32, i32), ParseInternalError> {
        if seconds_str.contains(".") {
            let split: Vec<&str> = seconds_str.split(".").collect();
            let (i, f): (&str, &str) = (split[0], split[1]);

            let i_parse = i.parse::<i32>()?;
            let f_parse = ljust(f, 6, '0').parse::<i32>()?;
            Ok((i_parse, f_parse))
        } else {
            Ok((seconds_str.parse::<i32>()?, 0))
        }
    }

    fn find_hms_index(
        &self,
        idx: usize,
        tokens: &Vec<String>,
        info: &ParserInfo,
        allow_jump: bool,
    ) -> Option<usize> {
        let len_l = tokens.len();
        let mut hms_idx = None;

        // There's a super weird edge case that can happen
        // because Python safely handles negative array indices,
        // and Rust (because of usize) does not.
        let idx_minus_two = if idx == 1 && len_l > 0 {
            len_l - 1
        } else if idx == 0 && len_l > 1 {
            len_l - 2
        } else if idx > 1 {
            idx - 2
        } else if len_l == 0{
            panic!("Attempting to find_hms_index() wih no tokens.");
        } else {
            0
        };

        if idx + 1 < len_l && info.get_hms(&tokens[idx + 1]).is_some() {
            hms_idx = Some(idx + 1)
        } else if allow_jump && idx + 2 < len_l && tokens[idx + 1] == " "
            && info.get_hms(&tokens[idx + 2]).is_some()
        {
            hms_idx = Some(idx + 2)
        } else if idx > 0 && info.get_hms(&tokens[idx - 1]).is_some() {
            hms_idx = Some(idx - 1)
        } else if len_l > 0 && idx > 0 && idx == len_l - 1 && tokens[idx - 1] == " "
            && info.get_hms(&tokens[idx_minus_two]).is_some()
        {
            hms_idx = Some(idx - 2)
        }

        hms_idx
    }

    fn parse_hms(
        &self,
        idx: usize,
        tokens: &Vec<String>,
        info: &ParserInfo,
        hms_index: Option<usize>,
    ) -> (usize, Option<usize>) {
        if hms_index.is_none() {
            (idx, None)
        } else if hms_index.unwrap() > idx {
            (
                hms_index.unwrap(),
                info.get_hms(&tokens[hms_index.unwrap()]),
            )
        } else {
            (
                idx,
                info.get_hms(&tokens[hms_index.unwrap()]).map(|u| u + 1),
            )
        }
    }

    fn assign_hms(&self, res: &mut ParsingResult, value_repr: &str, hms: usize) {
        let value = self.to_decimal(value_repr);

        if hms == 0 {
            res.hour = Some(value.to_i64().unwrap() as i32);
            if !close_to_integer(&value) {
                res.minute = Some((*SIXTY * (value % *ONE)).to_i64().unwrap() as i32);
            }
        } else if hms == 1 {
            let (min, sec) = self.parse_min_sec(value);
            res.minute = Some(min);
            res.second = sec;
        } else if hms == 2 {
            let (sec, micro) = self.parsems(value_repr).unwrap();
            res.second = Some(sec);
            res.microsecond = Some(micro);
        }
    }

    fn to_decimal(&self, value: &str) -> Decimal {
        // TODO: Justify unwrap
        Decimal::from_str(value).unwrap()
    }

    fn parse_min_sec(&self, value: Decimal) -> (i32, Option<i32>) {
        // UNWRAP: i64 guaranteed to be fine because of preceding floor
        let minute = value.floor().to_i64().unwrap() as i32;
        let mut second = None;

        let sec_remainder = value - value.floor();
        if sec_remainder != *ZERO {
            second = Some((*SIXTY * sec_remainder).floor().to_i64().unwrap() as i32);
        }

        (minute, second)
    }

    fn recombine_skipped(&self, skipped_idxs: Vec<usize>, tokens: Vec<String>) -> Vec<String> {
        let mut skipped_tokens: Vec<String> = vec![];

        let mut sorted_idxs = skipped_idxs.clone();
        sorted_idxs.sort();

        for (i, idx) in sorted_idxs.iter().enumerate() {
            if i > 0 && idx - 1 == skipped_idxs[i - 1] {
                // UNWRAP: Having an initial value and unconditional push at end guarantees value
                let mut t = skipped_tokens.pop().unwrap();
                t.push_str(tokens[idx.clone()].as_ref());
                skipped_tokens.push(t);
            } else {
                skipped_tokens.push(tokens[idx.clone()].to_owned());
            }
        }

        skipped_tokens
    }
}

fn close_to_integer(value: &Decimal) -> bool {
    value % *ONE == *ZERO
}

fn ljust(s: &str, chars: usize, replace: char) -> String {
    if s.len() >= chars {
        s[..chars].to_owned()
    } else {
        format!("{}{}", s, replace.to_string().repeat(chars - s.len()))
    }
}

pub fn parse(timestr: &str) -> ParseResult<(NaiveDateTime, Option<FixedOffset>)> {
    let res = Parser::default().parse(
        timestr,
        None,
        None,
        false,
        false,
        None,
        false,
        HashMap::new(),
    )?;

    Ok((res.0, res.1))
}
