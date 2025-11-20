use thiserror::Error;

pub type Result<T> = std::result::Result<T, CronError>;

#[derive(Debug, Error)]
pub enum CronError {
    #[error("Invalid cron value.")]
    InvalidCronValue,
}
