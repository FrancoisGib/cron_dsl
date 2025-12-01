use crate::{
    cron::Cron, task::CronTask, value::{interval, range}
    task::CronTask,
    value::{from, interval, on, range},
};

pub mod cron;
pub mod error;
pub mod task;
pub mod value;

// const FORMAT_NO_FRAC: &str = "%Y-%m-%d %H:%M:%S";

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

    let t = CronTask::builder()
        .minutes(from(10, 30).every(5))
        .hour(on(5).and(18))
        .build()
        .unwrap();
    println!("{}", t);

    println!("next {}", t.next_occurrence());

    // let local_time = Local::now();
    // let date_str = "2025-11-24 15:43:07";
    // let date = Local
    //     .from_local_datetime(&NaiveDateTime::parse_from_str(&date_str, FORMAT_NO_FRAC).unwrap())
    //     .unwrap();

    // let matches = cron_task.matches(date);

    // println!("match {}", matches);
}
