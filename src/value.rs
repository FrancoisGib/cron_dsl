use std::fmt::Display;

use cronvalue::FromTuple;

use crate::{error::{CronError, Result}};

#[derive(Debug, FromTuple)]
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

pub fn range(bot: u8, top: u8) -> CronValue {
    CronValue::Range(bot, top)
}

pub fn interval<T: Into<CronValue>>(base: T, step: u8) -> CronValue {
    CronValue::Interval(Box::new(base.into()), step)
}

pub fn value(value: u8) -> CronValue {
    value.into()
}

pub fn all() -> CronValue {
    CronValue::All
}
