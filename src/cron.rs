use chrono::{DateTime, Local};

use crate::task::CronTask;

#[derive(Debug, Default)]
pub struct Cron {
    tasks: Vec<CronTask>,
}

impl Cron {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn is_planified_at(&self, date: DateTime<Local>) -> bool {
        self.into_iter().any(|task| task.matches(date))
    }

    pub fn get_all_planified_at(&self, date: DateTime<Local>) -> Vec<&CronTask> {
        self.into_iter().filter(|task| task.matches(date)).collect()
    }
}

impl<'a> IntoIterator for &'a Cron {
    type Item = &'a CronTask;
    type IntoIter = std::slice::Iter<'a, CronTask>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.tasks.iter()
    }
}
