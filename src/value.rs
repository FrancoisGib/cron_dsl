use cronvalue::FromTuple;
use std::fmt::Display;

use crate::error::{CronError, Result};

#[derive(Debug, FromTuple, Clone)]
pub enum CronValue {
    Range(u8, u8),
    Value(u8),
    List(Vec<CronValue>),
    Interval(Box<CronValue>, u8),
    All,
}

impl Default for CronValue {
    fn default() -> Self {
        CronValue::All
    }
}

impl From<u8> for CronValue {
    fn from(value: u8) -> Self {
        CronValue::Value(value)
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
            CronValue::Range(begin, end) => write!(f, "{}-{}", *begin, *end),
            CronValue::Value(v) => write!(f, "{}", *v),
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
            CronValue::Range(_, _) | CronValue::All => match step.into() {
                CronValue::Value(v) => CronValue::Interval(Box::new(self), v),
                _ => self,
            },
            _ => self,
        }
    }

    pub fn from(self, a: u8, b: u8) -> Self {
        CronValue::Range(a, b).intersect(self)
    }

    fn intersect(self, other: Self) -> Self {
        match (self, other.clone()) {
            (CronValue::Range(a1, b1), CronValue::Range(a2, b2)) => {
                CronValue::Range(a1.max(a2), b1.min(b2))
            }

            (CronValue::Range(a, b), CronValue::Value(v))
            | (CronValue::Value(v), CronValue::Range(a, b)) => {
                if a <= v && v <= b {
                    CronValue::Value(v)
                } else {
                    CronValue::List(vec![])
                }
            }

            (CronValue::Range(a, b), CronValue::List(list))
            | (CronValue::List(list), CronValue::Range(a, b)) => {
                CronValue::List(list.into_iter().filter(|v| v.matches_in(a, b)).collect())
            }

            _ => other, // all
        }
    }

    fn matches_in(&self, a: u8, b: u8) -> bool {
        match self {
            CronValue::Value(v) => *v >= a && *v <= b,
            CronValue::Range(start, end) => *start >= a && *end <= b,
            CronValue::List(vals) => vals.iter().all(|v| v.matches_in(a, b)),
            CronValue::All => true,
            CronValue::Interval(_, _) => panic!("matches_in() should not be used on Interval"),
        }
    }

    pub fn verify_for_minute(&self) -> Result<()> {
        self.verify()?;
        match self {
            CronValue::Range(begin, end) => {
                if *begin < 60 && *end < 60 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Interval(base, step) => {
                base.as_ref().verify()?;
                if *step < 60 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Value(v) => {
                if *v < 60 {
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

    pub fn verify_for_hour(&self) -> Result<()> {
        self.verify()?;
        match self {
            CronValue::Range(begin, end) => {
                if *begin < 24 && *end < 24 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Interval(base, step) => {
                base.as_ref().verify()?;
                if *step < 24 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Value(v) => {
                if *v < 24 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::List(l) => l
                .iter()
                .map(|v| v.verify_for_hour())
                .fold(Ok(()), |acc, v| if v.is_err() { v } else { acc }),
            _ => Ok(()),
        }
    }

    pub fn verify_for_month(&self) -> Result<()> {
        self.verify()?;
        match self {
            CronValue::Range(begin_ref, end_ref) => {
                let begin = *begin_ref;
                let end = *end_ref;
                if begin > 0 && begin <= 12 && end > 0 && end <= 12 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Interval(base, step_ref) => {
                base.as_ref().verify()?;
                let step = *step_ref;
                if step > 0 && step <= 12 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Value(v_ref) => {
                let v = *v_ref;
                if v > 0 && v <= 12 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::List(l) => l
                .iter()
                .map(|v| v.verify_for_month())
                .fold(Ok(()), |acc, v| if v.is_err() { v } else { acc }),
            _ => Ok(()),
        }
    }

    pub fn verify_for_day(&self) -> Result<()> {
        self.verify()?;
        match self {
            CronValue::Range(begin_ref, end_ref) => {
                let begin = *begin_ref;
                let end = *end_ref;
                if begin > 0 && begin <= 31 && end > 0 && end <= 31 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Interval(base, step_ref) => {
                base.as_ref().verify()?;
                let step = *step_ref;
                if step > 0 && step <= 31 {
                    Ok(())
                } else {
                    Err(CronError::InvalidCronValue)
                }
            }
            CronValue::Value(v_ref) => {
                let v = *v_ref;
                if v > 0 && v <= 31 {
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
            CronValue::Range(begin, end) => *begin <= value && value <= *end,
            CronValue::Value(v) => *v == value,
            CronValue::List(cron_values) => cron_values.iter().any(|v| v.matches(value)),
            CronValue::Interval(base, step) => match base.as_ref() {
                CronValue::All => value % step == 0,
                CronValue::Range(begin, end) => {
                    if value < *begin || value > *end {
                        return false;
                    }
                    (value - begin) % step == 0
                }
                CronValue::Value(v) => value == *v && value % step == 0,
                CronValue::List(list) => list
                    .iter()
                    .any(|v| CronValue::Interval(v.clone().into(), *step).matches(value)),
                _ => false,
            },
            CronValue::All => true,
        }
    }

    pub fn min_value(&self) -> Option<u8> {
        match self {
            CronValue::Value(v) => Some(*v),
            CronValue::Range(a, _) => Some(*a),
            CronValue::Interval(base, step) => base.min_value().map(|v| v - (v % step)),
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

pub fn range(bot: u8, top: u8) -> CronValue {
    CronValue::Range(bot, top)
}

pub fn interval<T: Into<CronValue>>(base: T, step: u8) -> CronValue {
    CronValue::Interval(Box::new(base.into()), step)
}

pub fn every(step: u8) -> CronValue {
    CronValue::Interval(Box::new(CronValue::All), step)
}

pub fn from(begin: u8, end: u8) -> CronValue {
    CronValue::Range(begin, end)
}

pub fn all() -> CronValue {
    CronValue::All
}

pub fn on(value: u8) -> CronValue {
    CronValue::Value(value)
}
