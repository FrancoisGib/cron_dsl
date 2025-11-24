use crate::{
    cron::Cron, error::Result, task::CronTask, value::{interval, range, value}
};

pub mod cron;
pub mod error;
pub mod task;
pub mod value;

fn main() -> Result<()> {
    let mut cron = Cron::new();
    let task = CronTask::builder()
        .minutes((range(10..20), value(10), interval(10, 20)))
        .hour(10)
        .build()?;

    cron.add_task(task);

    println!("{cron:?}");

    Ok(())
}
