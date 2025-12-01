use std::{fmt::Display, ops::Range};

use chrono::{Month, Weekday};
use cronvalue::FromTuple;

use crate::error::{CronError, Result};

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
            Self::Month(m) => write!(f, "{m:?}"),
            Self::Number(n) => write!(f, "{n}"),
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

impl From<ValueKind> for u8 {
    fn from(value: ValueKind) -> Self {
        match value {
            ValueKind::Day(d) => d as u8,
            ValueKind::Month(m) => m as u8,
            ValueKind::Number(n) => n as u8,
        }
    }
}

impl From<&ValueKind> for u8 {
    fn from(value: &ValueKind) -> Self {
        match value {
            ValueKind::Day(d) => *d as u8,
            ValueKind::Month(m) => *m as u8,
            ValueKind::Number(n) => *n as u8,
        }
    }
}

impl From<ValueKind> for usize {
    fn from(value: ValueKind) -> Self {
        match value {
            ValueKind::Day(d) => d as usize,
            ValueKind::Month(m) => m as usize,
            ValueKind::Number(n) => n as usize,
        }
    }
}

impl From<&ValueKind> for usize {
    fn from(value: &ValueKind) -> Self {
        match value {
            ValueKind::Day(d) => *d as usize,
            ValueKind::Month(m) => *m as usize,
            ValueKind::Number(n) => *n as usize,
        }
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

impl CronValue {
    pub const ALL: Self = Self::All;

    pub fn matches(&self, value: u8) -> bool {
        match self {
            CronValue::Range(r) => r.start <= value && r.end >= value,
            CronValue::Value(v) => u8::from(v) == value,
            CronValue::List(cron_values) => cron_values.iter().any(|v| v.matches(value)),
            CronValue::Interval(cron_value, interval) => match cron_value.as_ref() {
                CronValue::All => true,
                CronValue::Range(r) => {
                    for v in r.clone().step_by(interval.into()) {
                        if value == v {
                            return true;
                        }
                    }
                    false
                }
                base => base.matches(value) && value % u8::from(interval) == 0,
            },
            CronValue::All => true,
        }
    }

    pub fn verify(&self, min: u8, max: u8) -> Result<()> {
        match self {
            CronValue::Range(r) => {
                if r.start < r.end && r.start >= min && r.end <= max {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Interval(_, v) => {
                let v: u8 = v.into();

                if v < max {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Value(v) => {
                let v: u8 = v.into();

                if v < max {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::List(l) => l
                .iter()
                .map(|v| v.verify(min, max))
                .fold(Ok(()), |acc, v| if v.is_err() { v } else { acc }),
            _ => Ok(()),
        }
    }
}

pub fn range(r: Range<u8>) -> CronValue {
    r.into()
}

pub fn interval<T: Into<CronValue>>(base: T, step: u8) -> CronValue {
    CronValue::Interval(Box::new(base.into()), step.into())
}

pub fn value(value: impl Into<CronValue>) -> CronValue {
    value.into()
}
