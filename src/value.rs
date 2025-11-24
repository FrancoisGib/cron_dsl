use std::{fmt::{Display}, ops::Range};

use cronvalue::FromTuple;
use time::{Weekday, Month};

use crate::{error::{CronError, Result}};

#[derive(Debug, PartialEq)]
pub enum ValueKind {
    Day(Weekday),
    Month(Month),
    Number(u8),
}

impl Display for ValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Day(d) => write!(f, "{d}"),
            Self::Month(m) => write!(f, ""),
            Self::Number(n) => write!(f, "")
        }
    }
}

impl From<Weekday> for ValueKind {
    fn from(value: Weekday) -> Self {
        Self::Day(value)
    }
}

impl From<Month> for ValueKind {
    fn from(value: Month) -> Self {
        Self::Month(value)
    }
}

impl From<u8> for ValueKind {
    fn from(value: u8) -> Self {
        Self::Number(value)
    }
}

#[derive(Debug, FromTuple)]
pub enum CronValue {
    Range(Range<u8>),
    Value(ValueKind),
    List(Vec<CronValue>),
    Interval(Box<CronValue>, ValueKind),
    All,
}

impl Default for CronValue {
    fn default() -> Self {
        CronValue::All
    }
}

impl Display for CronValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CronValue::Range(r) => write!(f, "{}-{}", r.start, r.end),
            CronValue::Value(v) => v.fmt(f),
            CronValue::List(cron_values) => {
                let fmt = cron_values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                write!(f, "{}", fmt)
            }
            CronValue::Interval(base, step) => write!(f, "{}/{}", base.as_ref().to_string(), step),
            CronValue::All => write!(f, "*"),
        }
    }
}

impl From<Range<u8>> for CronValue {
    fn from(value: Range<u8>) -> Self {
        Self::Range(value)
    }
}

impl From<u8> for CronValue {
    fn from(value: u8) -> Self {
        Self::Value(value.into())
    }
}

// impl CronValue {
//     pub const ALL: Self = Self::All;

//     pub fn verify_for_minute(&self) -> Result<()> {
//         self.verify()?;
//         match self {
//             CronValue::Range(r) => {
//                 if r.start < 60 && r.end < 60 {
//                     Ok(())
//                 } else {
//                     Err(CronError::InvalidCronValue)
//                 }
//             }
//             CronValue::Interval(base, step) => {
//                 base.as_ref().verify()?;
//                 if *step < 60 {
//                     Ok(())
//                 } else {
//                     Err(CronError::InvalidCronValue)
//                 }
//             }
//             CronValue::Value(v) => {
//                 if *v < 60 {
//                     Ok(())
//                 } else {
//                     Err(CronError::InvalidCronValue)
//                 }
//             }
//             CronValue::List(l) => l
//                 .iter()
//                 .map(|v| v.verify_for_minute())
//                 .fold(Ok(()), |acc, v| if v.is_err() { v } else { acc }),
//             _ => Ok(()),
//         }
//     }

    pub fn verify_for_week_day(&self) -> Result<()> {
        self.verify()?;
        match self {
            CronValue::Range(begin_ref, end_ref) => {
                let begin = *begin_ref;
                let end = *end_ref;
                if begin < 7 && end < 7 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Interval(base, step_ref) => {
                base.as_ref().verify()?;
                let step = *step_ref;
                if step < 7 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Value(v_ref) => {
                let v = *v_ref;
                if v < 7 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::List(l) => l
                .iter()
                .map(|v| v.verify_for_day())
                .fold(Ok(()), |acc, v| if v.is_err() { v } else { acc }),
            _ => Ok(()),
        }
    }

    fn verify(&self) -> Result<()> {
        match self {
            CronValue::Range(begin, end) => {
                if begin < end {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Interval(base, _) => base.as_ref().verify(),
            CronValue::List(l) => l
                .iter()
                .map(|v| v.verify())
                .fold(Ok(()), |acc, v| if v.is_err() { v } else { acc }),
            _ => Ok(()),
        }
    }

    pub fn matches(&self, value: u8) -> bool {
        match self {
            CronValue::Range(begin, end) => *begin <= value && *end >= value,
            CronValue::Value(v) => *v == value,
            CronValue::List(cron_values) => cron_values.iter().any(|v| v.matches(value)),
            CronValue::Interval(cron_value, interval) => {
                match cron_value.as_ref() {
                    CronValue::All => true,
                    CronValue::Range(begin, end) => {
                        for v in (*begin..*end).step_by(*interval as usize) {
                            if value == v { return true }
                        }
                        false
                    }
                    base => base.matches(value) && value % interval == 0,
                }
            }
            CronValue::All => true,
        }
    }
}

//     fn verify(&self) -> Result<()> {
//         match self {
//             CronValue::Range(begin, end) => {
//                 if begin < end {
//                     Ok(())
//                 } else {
//                     Err(CronError::InvalidCronValue)
//                 }
//             }
//             CronValue::Interval(base, _) => base.as_ref().verify(),
//             CronValue::List(l) => l
//                 .iter()
//                 .map(|v| v.verify())
//                 .fold(Ok(()), |acc, v| if v.is_err() { v } else { acc }),
//             _ => Ok(()),
//         }
//     }
// }

pub fn range(r: Range<u8>) -> CronValue {
    r.into()
}

pub fn interval<T: Into<CronValue>>(base: T, step: u8) -> CronValue {
    CronValue::Interval(Box::new(base.into()), step.into())
}

pub fn value(value: u8) -> CronValue {
    value.into()
}