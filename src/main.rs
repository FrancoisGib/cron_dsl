use crate::{
    task::CronTask,
    value::{interval, range, value},
};

pub mod error;
pub mod task;
pub mod value;

fn main() {
    let cron_task = CronTask::builder()
        .minutes((range(10, 20), value(10), interval(10, 20)))
        .hour(10)
        .build()
        .unwrap();
    println!("{}", cron_task);
}
