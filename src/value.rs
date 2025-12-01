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
                base.as_ref().verify(0, 60)?;

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
 
pub fn from<T: Into<CronValue>, Y: Into<CronValue>>(begin: T, end: Y) -> FromState {
    let v: u8 = end.into().into();
    let end = CronValue::Value(ValueKind::Number(v + 1));
    FromState { value: begin.into()..end }
}

pub fn all() -> CronValue {
    CronValue::All
}

pub fn on(value: u8) -> OnState {
    OnState { value: CronValue::Value(value.into()) }
}

pub struct OnState {
    value: CronValue
}

impl Into<CronValue> for OnState {
    fn into(self) -> CronValue {
        self.value
    }
}

impl OnState {
    pub fn or(self, value: u8) -> Self {
        Self { value: CronValue::List(vec![self.value, value.into()]) }
    }
}

pub struct FromState {
    value: Range<CronValue>
}

impl FromState {
    pub fn every(self, value: u8) -> CronValue {
        let v: u8 = self.value.end.into();
        let v = CronValue::Value(ValueKind::Number(v + 1));
        CronValue::Interval(Box::new(CronValue::Range(self.value.start.into()..v.into())), value.into())
    }
}

impl From<Weekday> for CronValue {
    fn from(value: Weekday) -> Self {
        CronValue::Value(ValueKind::Day(value))
    }
}

impl From<Month> for CronValue {
    fn from(value: Month) -> Self {
        CronValue::Value(ValueKind::Month(value))
    }
}

impl Into<CronValue> for FromState {
    fn into(self) -> CronValue {
        CronValue::Range(self.value.start.into()..self.value.end.into())
    }
}

impl Into<u8> for CronValue {
    fn into(self) -> u8 {
        match self {
            CronValue::Value(value_kind) => match value_kind {
                ValueKind::Day(weekday) => weekday as u8,
                ValueKind::Month(month) => month as u8,
                ValueKind::Number(v) => v,
            },
            _ => unreachable!("Unreachable"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Month, Weekday};

    #[test]
    fn test_value_kind_display() {
        assert_eq!(ValueKind::Number(5).to_string(), "5");
        assert_eq!(ValueKind::Day(Weekday::Mon).to_string(), "Mon");
        assert_eq!(ValueKind::Month(Month::January).to_string(), "January");
    }

    #[test]
    fn test_value_kind_conversions() {
        let day = ValueKind::Day(Weekday::Mon);
        assert_eq!(u8::from(&day), Weekday::Mon as u8);
        
        let month = ValueKind::Month(Month::March);
        assert_eq!(u8::from(&month), Month::March as u8);
        
        let num = ValueKind::Number(42);
        assert_eq!(u8::from(&num), 42);
    }

    #[test]
    fn test_range_display() {
        let range = CronValue::Range(1..10);
        assert_eq!(range.to_string(), "1-10");
    }

    #[test]
    fn test_range_matches() {
        let range = CronValue::Range(5..15);
        assert!(range.matches(5));
        assert!(range.matches(10));
        assert!(range.matches(15));
        assert!(!range.matches(4));
        assert!(!range.matches(16));
    }

    #[test]
    fn test_range_min_value() {
        let range = CronValue::Range(10..20);
        assert_eq!(range.min_value(), Some(10));
    }

    #[test]
    fn test_value_matches() {
        let value = CronValue::Value(ValueKind::Number(42));
        assert!(value.matches(42));
        assert!(!value.matches(41));
        assert!(!value.matches(43));
    }

    #[test]
    fn test_value_min_value() {
        let value = CronValue::Value(ValueKind::Number(15));
        assert_eq!(value.min_value(), Some(15));
    }

    #[test]
    fn test_list_display() {
        let list = CronValue::List(vec![
            CronValue::Value(ValueKind::Number(1)),
            CronValue::Value(ValueKind::Number(5)),
            CronValue::Value(ValueKind::Number(10)),
        ]);
        assert_eq!(list.to_string(), "1,5,10");
    }

    #[test]
    fn test_list_matches() {
        let list = CronValue::List(vec![
            CronValue::Value(ValueKind::Number(5)),
            CronValue::Value(ValueKind::Number(10)),
            CronValue::Value(ValueKind::Number(15)),
        ]);
        assert!(list.matches(5));
        assert!(list.matches(10));
        assert!(list.matches(15));
        assert!(!list.matches(7));
        assert!(!list.matches(20));
    }

    #[test]
    fn test_list_min_value() {
        let list = CronValue::List(vec![
            CronValue::Value(ValueKind::Number(20)),
            CronValue::Value(ValueKind::Number(5)),
            CronValue::Value(ValueKind::Number(15)),
        ]);
        assert_eq!(list.min_value(), Some(5));
    }

    #[test]
    fn test_interval_all_display() {
        let interval = CronValue::Interval(
            Box::new(CronValue::All),
            ValueKind::Number(5),
        );
        assert_eq!(interval.to_string(), "*/5");
    }

    #[test]
    fn test_interval_range_display() {
        let interval = CronValue::Interval(
            Box::new(CronValue::Range(10..30)),
            ValueKind::Number(5),
        );
        assert_eq!(interval.to_string(), "10-30/5");
    }

    #[test]
    fn test_interval_all_matches() {
        let interval = CronValue::Interval(
            Box::new(CronValue::All),
            ValueKind::Number(5),
        );
        assert!(interval.matches(0));
        assert!(interval.matches(5));
        assert!(interval.matches(10));
        assert!(interval.matches(15));
        assert!(!interval.matches(3));
        assert!(!interval.matches(7));
    }

    #[test]
    fn test_interval_range_matches() {
        let interval = CronValue::Interval(
            Box::new(CronValue::Range(10..30)),
            ValueKind::Number(5),
        );
        assert!(interval.matches(10));
        assert!(interval.matches(15));
        assert!(interval.matches(20));
        assert!(interval.matches(25));
        assert!(interval.matches(30));
        assert!(!interval.matches(5));
        assert!(!interval.matches(12));
        assert!(!interval.matches(35));
    }

    #[test]
    fn test_all_display() {
        let all = CronValue::All;
        assert_eq!(all.to_string(), "*");
    }

    #[test]
    fn test_all_matches() {
        let all = CronValue::All;
        assert!(all.matches(0));
        assert!(all.matches(15));
        assert!(all.matches(59));
        assert!(all.matches(255));
    }

    #[test]
    fn test_all_min_value() {
        let all = CronValue::All;
        assert_eq!(all.min_value(), Some(0));
    }

    #[test]
    fn test_combine_values() {
        let value = on(5).or(10).into();
        
        match value {
            CronValue::List(ref list) => {
                assert_eq!(list.len(), 2);
            }
            _ => panic!("Expected List variant"),
        }
        
        assert!(value.matches(5));
        assert!(value.matches(10));
    }

    #[test]
    fn test_and_with_all_returns_all() {
        let all = CronValue::All;
        let result = all.and(5);
        assert!(matches!(result, CronValue::All));
    }

    #[test]
    fn test_every_creates_interval() {
        let range = CronValue::Range(0..59);
        let interval = range.every(5);
        
        match interval {
            CronValue::Interval(_, ref step) => {
                assert_eq!(u8::from(step), 5);
            }
            _ => panic!("Expected Interval variant"),
        }
    }

    #[test]
    fn test_every_with_all() {
        let all = CronValue::All;
        let interval = all.every(10);
        
        assert!(interval.matches(0));
        assert!(interval.matches(10));
        assert!(interval.matches(20));
        assert!(!interval.matches(5));
    }

    #[test]
    fn test_verify_for_minute_valid() {
        let value = value(30);
        assert!(value.verify_for_minute().is_ok());
        
        let range = CronValue::Range(0..59);
        assert!(range.verify_for_minute().is_ok());
        
        let interval = CronValue::Interval(
            Box::new(CronValue::All),
            ValueKind::Number(15),
        );
        assert!(interval.verify_for_minute().is_ok());
    }

    #[test]
    fn test_verify_for_minute_invalid() {
        let value = value(60);
        assert!(value.verify_for_minute().is_err());
        
        let range: CronValue = from(0, 60).into();
        assert!(range.verify_for_minute().is_err());
        
        let interval = interval(
            all(),
            60,
        );
        assert!(interval.verify_for_minute().is_err());
    }

    #[test]
    fn test_verify_range_valid() {
        let range = range(5..20);
        assert!(range.verify(0, 30).is_ok());
    }

    #[test]
    fn test_verify_range_invalid() {
        let range_value = range(5..20);
        assert!(range_value.verify(10, 15).is_err());
        
        let invalid_range = range(20..5);
        assert!(invalid_range.verify(0, 30).is_err());
    }

    #[test]
    fn test_verify_value() {
        let value = value(15);
        assert!(value.verify(0, 30).is_ok());
        assert!(value.verify(0, 10).is_err());
    }

    #[test]
    fn test_next_value_range() {
        let range = range(10..20);
        assert_eq!(range.next_value(5, 30), Some(10));
        assert_eq!(range.next_value(15, 30), Some(15));
        assert_eq!(range.next_value(21, 30), None);
    }

    #[test]
    fn test_next_value_interval() {
        let interval = interval(
            all(),
            5,
        );
        assert_eq!(interval.next_value(3, 30), Some(5));
        assert_eq!(interval.next_value(7, 30), Some(10));
        assert_eq!(interval.next_value(10, 30), Some(10));
    }

    #[test]
    fn test_next_value_list() {
        let list = CronValue::List(vec![
            value(5),
            value(15),
            value(25),
        ]);
        assert_eq!(list.next_value(0, 30), Some(5));
        assert_eq!(list.next_value(10, 30), Some(15));
        assert_eq!(list.next_value(20, 30), Some(25));
        assert_eq!(list.next_value(26, 30), None);
    }

    #[test]
    fn test_range_function() {
        let r = range(5..15);
        assert!(r.matches(10));
        assert!(!r.matches(20));
    }

    #[test]
    fn test_interval_function() {
        let i = interval(all(), 5);
        assert!(i.matches(0));
        assert!(i.matches(10));
        assert!(!i.matches(7));
    }

    #[test]
    fn test_every_function() {
        let e = every(10u8);
        assert!(e.matches(0));
        assert!(e.matches(10));
        assert!(e.matches(20));
        assert!(!e.matches(5));
    }

    #[test]
    fn test_all_function() {
        let a = all();
        assert!(a.matches(0));
        assert!(a.matches(42));
        assert!(a.matches(255));
    }

    #[test]
    fn test_on_function() {
        let on_state = on(15);
        let cron_value: CronValue = on_state.into();
        assert!(cron_value.matches(15));
        assert!(!cron_value.matches(14));
    }

    #[test]
    fn test_on_or() {
        let on_state = on(5).or(10);
        let cron_value: CronValue = on_state.into();
        assert!(cron_value.matches(5));
        assert!(cron_value.matches(10));
        assert!(!cron_value.matches(7));
    }

    #[test]
    fn test_from_function() {
        let from_state = from(10u8, 20u8);
        let cron_value: CronValue = from_state.into();
        assert!(cron_value.matches(10));
        assert!(cron_value.matches(15));
        assert!(cron_value.matches(21));
        assert!(!cron_value.matches(9));
        assert!(!cron_value.matches(22));
    }

    #[test]
    fn test_from_every() {
        let interval = from(10u8, 30u8).every(5);
        assert!(interval.matches(10));
        assert!(interval.matches(15));
        assert!(interval.matches(20));
        assert!(interval.matches(25));
        assert!(interval.matches(30));
        assert!(!interval.matches(12));
        assert!(!interval.matches(32));
    }

    #[test]
    fn test_complex_cron_expression() {
        let expr = every(5u8);
        assert!(expr.matches(0));
        assert!(expr.matches(5));
        assert!(expr.matches(10));
        assert!(!expr.matches(3));
    }

    #[test]
    fn test_weekday_conversion() {
        let monday = CronValue::from(Weekday::Mon);
        match monday {
            CronValue::Value(ValueKind::Day(d)) => {
                assert_eq!(d, Weekday::Mon);
            }
            _ => panic!("Expected Day variant"),
        }
    }

    #[test]
    fn test_month_conversion() {
        let january = CronValue::from(Month::January);
        match january {
            CronValue::Value(ValueKind::Month(m)) => {
                assert_eq!(m, Month::January);
            }
            _ => panic!("Expected Month variant"),
        }
    }
}