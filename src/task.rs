use std::{fmt::Display, path::PathBuf};

use chrono::{DateTime, Datelike, Local, Month, Timelike, Weekday};

use crate::{error::Result, value::CronValue};

#[derive(Debug, Default)]
pub struct CronTask {
    minute: CronValue,
    hour: CronValue,
    month_day: CronValue,
    month: CronValue,
    week_day: CronValue,
    path: PathBuf,
}

impl Display for CronTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "m{} h{} md{} m{} wd{} p{:?}",
            self.minute, self.hour, self.month_day, self.month, self.week_day, self.path
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
        path: PathBuf,
    ) -> Self {
        CronTask {
            minute,
            hour,
            month_day,
            month,
            week_day,
            path,
        }
    }

    pub fn builder() -> CronTaskBuilder {
        CronTaskBuilder::default()
    }

    fn verify(&self) -> Result<()> {
        self.minute.verify(0, 60)?;
        self.hour.verify(0, 24)?;
        self.month_day.verify(0, 31)?;
        self.month.verify(0, 12)?;
        self.week_day.verify(0, 6)?;
        
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
            value.path,
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
    path: PathBuf,
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

    pub fn path(mut self, path: String) -> Self {
        self.path = path.into();
        self
    }

    pub fn build(self) -> Result<CronTask> {
        let task = CronTask::from(self);

        task.verify().map(|_| task)
    }
}
