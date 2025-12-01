use std::{fmt::Display, path::PathBuf};

use chrono::{DateTime, Datelike, Local, NaiveDate, Timelike, TimeZone};

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
            "{} {} {} {} {} {:?}",
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

    pub fn next_occurrence(&self) -> DateTime<Local> {
        let from = Local::now();
        self.try_next_occurrence(from)
            .expect("no future occurrence found for valid cron expression")
    }

    pub fn try_next_occurrence(&self, from: DateTime<Local>) -> Option<DateTime<Local>> {
        let mut year = from.year();
        let mut month = from.month() as u8;
        let mut day = from.day() as u8;
        let mut hour = from.hour() as u8;
        let mut min = from.minute() as u8;

        loop {
            match self.month.next_value(month, 12) {
                Some(m) => month = m,
                None => {
                    year += 1;
                    month = self.month.min_value()?;
                    day = 1;
                    hour = 0;
                    min = 0;
                    continue;
                }
            }

            let mut found_day = None;

            for d in day..=30 as u8 {
                let wd = NaiveDate::from_ymd_opt(year, month as u32, d as u32)?.weekday() as u8;

                if self.month_day.matches(d) && self.week_day.matches(wd) {
                    found_day = Some(d);
                    break;
                }
            }

            let d = match found_day {
                Some(v) => v,
                None => {
                    month += 1;
                    day = 1;
                    hour = 0;
                    min = 0;
                    continue;
                }
            };
            day = d;

            match self.hour.next_value(hour, 23) {
                Some(h) => hour = h,
                None => {
                    day += 1;
                    hour = 0;
                    min = 0;
                    continue;
                }
            }

            match self.minute.next_value(min, 59) {
                Some(m) => min = m,
                None => {
                    hour += 1;
                    min = 0;
                    continue;
                }
            }

            if let Some(date) = NaiveDate::from_ymd_opt(year, month as u32, day as u32) {
                if let Some(dt) = date.and_hms_opt(hour as u32, min as u32, 0) {
                    let local = Local.from_local_datetime(&dt).single()?;
                    if local > from {
                        return Some(local);
                    }
                }
            }

            min += 1;
        }
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
