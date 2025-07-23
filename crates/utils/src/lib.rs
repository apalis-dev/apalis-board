use std::time::Duration;

use apalis_core::{backend::Stat, request::State};
use serde::{Deserialize, Serialize};

pub mod sse;

#[derive(Deserialize, Debug, Default)]
pub struct Filter {
    #[serde(default)]
    pub status: State,
    pub page: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Layer {
    Retry { retries: u64 },
    Timeout { duration: Duration },
    LoadShed,
    RateLimit { num: u64, per: Duration },
    ConcurrencyLimit { max: usize },
    Buffer { bound: usize },
    Sentry { dsn: String },
    Prometheus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetJobsResult<T> {
    pub stats: Stat,
    pub jobs: Vec<T>,
}
