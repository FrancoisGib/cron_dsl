use chrono::{Local, NaiveDateTime, TimeZone};

use crate::{
    cron::Cron, task::CronTask, value::{interval, range}
};

pub mod cron;
pub mod error;
pub mod task;
pub mod value;

const FORMAT_NO_FRAC: &str = "%Y-%m-%d %H:%M:%S";

fn main() {
    let mut cron = Cron::new();
    let cron_task = CronTask::builder()
        .minutes(range(1..59))
        .hour(15)
        .month(interval(range(1..12), 5))
        .week_day(0)
        .path("path".to_string())
        .build()
        .unwrap();
    println!("{}", cron_task);

    cron.add_task(cron_task);
    
    // let local_time = Local::now();
    // let date_str = "2025-11-24 15:43:07";
    // let date = Local.from_local_datetime(&NaiveDateTime::parse_from_str(&date_str, FORMAT_NO_FRAC).unwrap()).unwrap();

    // let matches = cron_task.matches(date);



    // println!("match {}", matches);
}
