use chrono::{Local, NaiveDateTime, TimeZone};

use crate::{
    task::CronTask,
    value::{interval, range},
};

pub mod error;
pub mod task;
pub mod value;
pub mod cron;

const FORMAT_NO_FRAC: &str = "%Y-%m-%d %H:%M:%S";

fn main() {
    let cron_task = CronTask::builder()
        .minutes(range(1, 59))
        .hour(15)
        .month(interval(range(1, 12), 5))
        .week_day(0)
        .build()
        .unwrap();
    println!("{}", cron_task);
    
    // let local_time = Local::now();
    let date_str = "2025-11-24 15:43:07";
    let date = Local.from_local_datetime(&NaiveDateTime::parse_from_str(&date_str, FORMAT_NO_FRAC).unwrap()).unwrap();

    let matches = cron_task.matches(date);



    println!("match {}", matches);
}
