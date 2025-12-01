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

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{DateTime, Local, TimeZone};

    use crate::value::{all, every, on};

    fn make_datetime(year: i32, month: u32, day: u32, hour: u32, min: u32) -> DateTime<Local> {
        Local
            .with_ymd_and_hms(year, month, day, hour, min, 0)
            .unwrap()
    }

    fn make_simple_task() -> CronTask {
        CronTask::builder()
            .minutes(all())
            .hour(all())
            .month_day(all())
            .month(all())
            .week_day(all())
            .path("/usr/bin/test".to_string())
            .build()
            .unwrap()
    }

    #[test]
    fn test_new_cron() {
        let cron = Cron::new();

        assert_eq!(cron.tasks.len(), 0);
    }

    #[test]
    fn test_default_cron() {
        let cron = Cron::default();

        assert_eq!(cron.tasks.len(), 0);
    }

    #[test]
    fn test_add_single_task() {
        let mut cron = Cron::new();
        let task = make_simple_task();

        cron.add_task(task);

        assert_eq!(cron.tasks.len(), 1);
    }

    #[test]
    fn test_add_multiple_tasks() {
        let mut cron = Cron::new();

        cron.add_task(make_simple_task());
        cron.add_task(make_simple_task());
        cron.add_task(make_simple_task());

        assert_eq!(cron.tasks.len(), 3);
    }

    #[test]
    fn test_new_cron_iter() {
        let cron = Cron::new();

        assert_eq!(cron.into_iter().count(), 0);
    }

    #[test]
    fn test_new_cron_empty_iter() {
        let cron = Cron::new();

        assert!(cron.into_iter().next().is_none());
    }

    #[test]
    fn test_iter_with_tasks() {
        let mut cron = Cron::new();

        cron.add_task(make_simple_task());
        cron.add_task(make_simple_task());

        let count = cron.into_iter().count();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_iter_multiple_times() {
        let mut cron = Cron::new();

        cron.add_task(make_simple_task());

        assert_eq!(cron.into_iter().count(), 1);
        assert_eq!(cron.into_iter().count(), 1);
    }

    #[test]
    fn test_iter_collect() {
        let mut cron = Cron::new();

        cron.add_task(make_simple_task());
        cron.add_task(make_simple_task());

        let tasks: Vec<_> = cron.into_iter().collect();

        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_is_planified_at_empty_cron() {
        let cron = Cron::new();
        let date = make_datetime(2024, 1, 1, 12, 0);

        assert!(!cron.is_planified_at(date));
    }

    #[test]
    fn test_is_planified_at_with_matching_task() {
        let mut cron = Cron::new();
        let task = CronTask::builder()
            .minutes(all())
            .hour(all())
            .month_day(all())
            .month(all())
            .week_day(all())
            .path("/usr/bin/test".to_string())
            .build()
            .unwrap();

        cron.add_task(task);

        let matching_date = make_datetime(2024, 6, 15, 12, 0);

        assert!(cron.is_planified_at(matching_date));
    }

    #[test]
    fn test_cron_task_builder_basic() {
        let task = CronTask::builder()
            .minutes(on(0))
            .hour(on(12))
            .month_day(on(15))
            .month(on(6))
            .week_day(all())
            .path("/usr/bin/test".to_string())
            .build();

        assert!(task.is_ok());
    }

    #[test]
    fn test_cron_task_matches() {
        let task = CronTask::builder()
            .minutes(on(30))
            .hour(on(14))
            .month_day(on(15))
            .month(on(6))
            .week_day(all())
            .path("/usr/bin/test".to_string())
            .build()
            .unwrap();

        let matching = make_datetime(2024, 6, 15, 14, 30);
        let not_matching = make_datetime(2024, 6, 15, 14, 31);

        assert!(task.matches(matching));
        assert!(!task.matches(not_matching));
    }

    #[test]
    fn test_cron_task_display() {
        let task = CronTask::builder()
            .minutes(on(0))
            .hour(on(12))
            .month_day(all())
            .month(all())
            .week_day(all())
            .path("/usr/bin/test".to_string())
            .build()
            .unwrap();

        let display = format!("{}", task);
        assert!(display.contains("0"));
        assert!(display.contains("12"));
        assert!(display.contains("*"));
        assert!(display.contains("/usr/bin/test"));
    }

    #[test]
    fn test_cron_task_with_interval() {
        let task = CronTask::builder()
            .minutes(every(5))
            .hour(all())
            .month_day(all())
            .month(all())
            .week_day(all())
            .path("/usr/bin/test".to_string())
            .build()
            .unwrap();

        assert!(task.matches(make_datetime(2024, 6, 15, 12, 0)));
        assert!(task.matches(make_datetime(2024, 6, 15, 12, 5)));
        assert!(task.matches(make_datetime(2024, 6, 15, 12, 10)));
        assert!(!task.matches(make_datetime(2024, 6, 15, 12, 3)));
    }
}
