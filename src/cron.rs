use crate::task::CronTask;

#[derive(Debug, Default)]
pub struct Cron {
    tasks: Vec<CronTask>,
}

impl Cron {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }
}

impl<'a> IntoIterator for &'a Cron {
    type Item = &'a CronTask;
    type IntoIter = std::slice::Iter<'a, CronTask>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.tasks.iter()
    }
}
