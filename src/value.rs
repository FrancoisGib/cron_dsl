use std::{fmt::Display, ops::Range};

use chrono::{Month, Weekday};
use cronvalue::FromTuple;

use crate::error::{CronError, Result};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, FromTuple, Clone)]
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

impl From<u8> for CronValue {
    fn from(value: u8) -> Self {
        CronValue::Value(value.into())
    }
}

impl<T> From<&[T]> for CronValue
where
    T: Into<CronValue> + Clone,
{
    fn from(value: &[T]) -> Self {
        CronValue::List(value.iter().map(|v| v.clone().into()).collect())
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

impl CronValue {
    pub fn and<T: Into<CronValue>>(self, value: T) -> CronValue {
        let mut values = match self {
            CronValue::List(cron_values) => cron_values,
            CronValue::All => return CronValue::All,
            v => vec![v],
        };
        values.push(value.into());
        CronValue::List(values)
    }

    pub fn every<T: Into<CronValue>>(self, step: T) -> Self {
        match self {
            CronValue::Range(_) | CronValue::All => match step.into() {
                CronValue::Value(v) => CronValue::Interval(Box::new(self), v),
                _ => self,
            },
            _ => self,
        }
    }

    pub fn from(self, r: Range<u8>) -> Self {
        CronValue::Range(r).intersect(self)
    }

    fn intersect(self, other: Self) -> Self {
        match (self, other.clone()) {
            (CronValue::Range(r1), CronValue::Range(r2)) => {
                CronValue::Range(r1.start.max(r2.start)..r1.end.min(r2.end))
            }

            (CronValue::Range(r), CronValue::Value(v))
            | (CronValue::Value(v), CronValue::Range(r)) => {
                if r.start <= u8::from(&v) && u8::from(&v) <= r.end {
                    CronValue::Value(v)
                } else {
                    CronValue::List(vec![])
                }
            }

            (CronValue::Range(r), CronValue::List(list))
            | (CronValue::List(list), CronValue::Range(r)) => {
                CronValue::List(list.into_iter().filter(|v| v.matches_in(r.start, r.end)).collect())
            }

            _ => other, // all
        }
    }

    fn matches_in(&self, a: u8, b: u8) -> bool {
        match self {
            CronValue::Value(v) => u8::from(v) >= a && u8::from(v) <= b,
            CronValue::Range(r) => r.start >= a && r.end <= b,
            CronValue::List(vals) => vals.iter().all(|v| v.matches_in(a, b)),
            CronValue::All => true,
            CronValue::Interval(_, _) => panic!("matches_in() should not be used on Interval"),
        }
    }

    pub fn verify_for_minute(&self) -> Result<()> {
        self.verify(0, 60)?;

        match self {
            CronValue::Range(r) => {
                if r.start < 60 && r.end < 60 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Interval(base, step) => {
                base.as_ref().verify(0, 60)?; // TODO

                if u8::from(step) < 60 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Value(v) => {
                if u8::from(v) < 60 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::List(l) => l
                .iter()
                .map(|v| v.verify_for_minute())
                .fold(Ok(()), |acc, v| if v.is_err() { v } else { acc }),
            _ => Ok(()),
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

    pub fn matches(&self, value: u8) -> bool {
        match self {
            CronValue::Range(r) => r.start <= value && value <= r.end,
            CronValue::Value(v) => u8::from(v) == value,
            CronValue::List(cron_values) => cron_values.iter().any(|v| v.matches(value)),
            CronValue::Interval(base, step) => match base.as_ref() {
                CronValue::All => value % u8::from(step) == 0,
                CronValue::Range(r) => {
                    if value < r.start|| value > r.end {
                        return false;
                    }
                    (value - r.start) % u8::from(step) == 0
                }
                CronValue::Value(v) => value == u8::from(v) && value % u8::from(step) == 0,
                CronValue::List(list) => list
                    .iter()
                    .any(|v| CronValue::Interval(v.clone().into(), step.clone()).matches(value)),
                _ => false,
            },
            CronValue::All => true,
        }
    }

    pub fn min_value(&self) -> Option<u8> {
        match self {
            CronValue::Value(v) => Some(u8::from(v)),
            CronValue::Range(r ) => Some(r.start),
            CronValue::Interval(base, step) => base.min_value().map(|v| v - (v % u8::from(step))),
            CronValue::List(list) => list.iter().filter_map(|v| v.min_value()).min(),
            CronValue::All => Some(0),
        }
    }

    pub fn next_value(&self, current: u8, max: u8) -> Option<u8> {
        for v in current..=max {
            if self.matches(v) {
                return Some(v);
            }
        }
        None
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
pub fn every(step: impl Into<ValueKind>) -> CronValue {
    CronValue::Interval(Box::new(CronValue::All), step.into())
}

pub fn from(begin: u8, end: u8) -> CronValue {
    CronValue::Range(begin..end)
}

pub fn all() -> CronValue {
    CronValue::All
}

pub fn on(value: u8) -> CronValue {
    CronValue::Value(value.into())
}
