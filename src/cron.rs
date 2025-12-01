use chrono::{DateTime, Local};

use crate::task::CronTask;

#[derive(Debug, Default)]
pub struct Cron {
    tasks: Vec<CronTask>,
}

impl Cron {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_task(&mut self, task: CronTask) {
        self.tasks.push(task);
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
