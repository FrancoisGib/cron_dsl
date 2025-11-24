use std::fmt::Display;

use chrono::{DateTime, Datelike, Local, Timelike};

use crate::{error::Result, value::CronValue};

#[derive(Debug, Default)]
pub struct CronTask {
    minute: CronValue,
    hour: CronValue,
    month_day: CronValue,
    month: CronValue,
    week_day: CronValue,
}

impl Display for CronTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.minute, self.hour, self.month_day, self.month, self.week_day
        )
    }
}

impl CronTask {
    pub fn new(
        minute: CronValue,
        hour: CronValue,
        month_day: CronValue,
        month: CronValue,
        week_day: CronValue,
    ) -> Self {
        CronTask {
            minute,
            hour,
            month_day,
            month,
            week_day,
        }
    }

    pub fn builder() -> CronTaskBuilder {
        CronTaskBuilder::default()
    }

    fn verify(&self) -> Result<()> {
        self.minute.verify_for_minute()?;
        self.hour.verify_for_hour()?;
        self.month_day.verify_for_day()?;
        self.month.verify_for_month()?;
        self.week_day.verify_for_week_day()?;
        Ok(())
    }

    pub fn matches(&self, date: DateTime<Local>) -> bool {
        self.week_day.matches(date.weekday() as u8)
        && self.month_day.matches(date.day() as u8)
        && self.hour.matches(date.hour() as u8)
        && self.month.matches(date.month() as u8)
        && self.minute.matches(date.minute() as u8)
    }
}

impl From<CronTaskBuilder> for CronTask {
    fn from(value: CronTaskBuilder) -> Self {
        CronTask::new(
            value.minute,
            value.hour,
            value.month_day,
            value.month,
            value.week_day,
        )
    }
}

#[derive(Debug, Default)]
pub struct CronTaskBuilder {
    minute: CronValue,
    hour: CronValue,
    month_day: CronValue,
    month: CronValue,
    week_day: CronValue,
}

impl CronTaskBuilder {
    pub fn minutes<T: Into<CronValue>>(mut self, value: T) -> Self {
        self.minute = value.into();
        self
    }

    pub fn hour<T: Into<CronValue>>(mut self, value: T) -> Self {
        self.hour = value.into();
        self
    }

    pub fn month_day<T: Into<CronValue>>(mut self, value: T) -> Self {
        self.month_day = value.into();
        self
    }

    pub fn month<T: Into<CronValue>>(mut self, value: T) -> Self {
        self.month = value.into();
        self
    }

    pub fn week_day<T: Into<CronValue>>(mut self, value: T) -> Self {
        self.week_day = value.into();
        self
    }

    pub fn build(self) -> Result<CronTask> {
        let task = CronTask::from(self);
        task.verify().map(|_| task)
    }
}
