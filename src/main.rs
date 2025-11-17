use std::str::FromStr;

use crate::error::{CronError, Result};

pub mod error;

#[derive(Debug)]
enum CronValue {
    Range(u8, u8),
    Value(u8),
    List(Vec<CronValue>),
    Interval(Box<CronValue>, u8),
    All
}

impl CronValue {
    pub fn seq(values: impl AsRef<[CronValue]>) {}
}

impl Default for CronValue {
    fn default() -> Self {
        CronValue::All
    }
}

impl FromStr for CronValue {
    type Err = CronError;
    
    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct CronTask {
    minute: CronValue,
    hour: CronValue,
    month_day: CronValue,
    month: CronValue,
    week_day: CronValue
}

impl CronTask {
    pub fn builder() -> CronTaskBuilder {
        CronTaskBuilder::default()
    }
}

impl From<CronTaskBuilder> for CronTask {
    fn from(value: CronTaskBuilder) -> Self {
        CronTask {
            minute: value.minute,
            hour: value.hour,
            month_day: value.month_day,
            month: value.month,
            week_day: value.week_day,
        }
    }
}

#[derive(Debug, Default)]
struct CronTaskBuilder {
    minute: CronValue,
    hour: CronValue,
    month_day: CronValue,
    month: CronValue,
    week_day: CronValue,
}

impl CronTaskBuilder {
    pub fn minutes(mut self, value: CronValue) -> Self {
        self.minute = value;
        self
    }

    pub fn hour(mut self, value: CronValue) -> Self {
        self.hour = value;
        self
    }

    pub fn month_day(mut self, value: CronValue) -> Self {
        self.month_day = value;
        self
    }

    pub fn month(mut self, value: CronValue) -> Self {
        self.month = value;
        self
    }

    pub fn week_day(mut self, value: CronValue) -> Self {
        self.week_day = value;
        self
    }

    pub fn build(self) -> CronTask {
        CronTask::from(self)
    }
}

fn main() {
    // let cron_task = CronTask::builder()
    //     .minutes(
    //         CronValue::Seq(
    //             CronValue::Op(
    //                 CronOp::And(a, b)
    //             ),
    //             CronValue::Op(
    //                 CronOp::Or(a, b)
    //             ),
    //         )
    //         seq(
    //             and(a,b),
    //             or(a, b)
    //         )
    // ).hour().build();

    // CronValue::seq((CronValue::All, CronValue::All, CronValue::All));
    
    // println!("{:?}", cron_task);
}
