use std::{fmt, str::FromStr};

use leptos::{
    prelude::{AnyView, IntoAny},
    IntoView,
};
use leptos_i18n::I18nContext;

use crate::i18n::{t, I18nKeys, Locale};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum KnownStatistic {
    RunningJobs,
    PendingJobs,
    FailedJobs,
    ActiveJobs,
    StaleRunningJobs,
    FailureRate,
    JobsPastHour,
    JobsToday,
    FailedJobsToday,
    AvgJobsPerMinutePastHour,
    TotalJobs,
    DoneJobs,
    CompletedJobs,
    KilledJobs,
    SuccessRate,
    AvgJobDurationMins,
    LongestRunningJobMins,
    QueueBacklog,
    JobsPast24Hours,
    JobsPast7Days,
    FailedJobsPast7Days,
    SuccessRatePast24h,
    AvgJobsPerHourPast24h,
    AvgJobsPerDayPast7d,
    MostRecentJob,
    OldestPendingJob,
    PeakHourJobs,
    DbPageSize,
    DbPageCount,
    DbSize,
}

impl KnownStatistic {

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RunningJobs => "RUNNING_JOBS",
            Self::PendingJobs => "PENDING_JOBS",
            Self::FailedJobs => "FAILED_JOBS",
            Self::ActiveJobs => "ACTIVE_JOBS",
            Self::StaleRunningJobs => "STALE_RUNNING_JOBS",
            Self::FailureRate => "FAILURE_RATE",
            Self::JobsPastHour => "JOBS_PAST_HOUR",
            Self::JobsToday => "JOBS_TODAY",
            Self::FailedJobsToday => "FAILED_JOBS_TODAY",
            Self::AvgJobsPerMinutePastHour => "AVG_JOBS_PER_MINUTE_PAST_HOUR",
            Self::TotalJobs => "TOTAL_JOBS",
            Self::DoneJobs => "DONE_JOBS",
            Self::CompletedJobs => "COMPLETED_JOBS",
            Self::KilledJobs => "KILLED_JOBS",
            Self::SuccessRate => "SUCCESS_RATE",
            Self::AvgJobDurationMins => "AVG_JOB_DURATION_MINS",
            Self::LongestRunningJobMins => "LONGEST_RUNNING_JOB_MINS",
            Self::QueueBacklog => "QUEUE_BACKLOG",
            Self::JobsPast24Hours => "JOBS_PAST_24_HOURS",
            Self::JobsPast7Days => "JOBS_PAST_7_DAYS",
            Self::FailedJobsPast7Days => "FAILED_JOBS_PAST_7_DAYS",
            Self::SuccessRatePast24h => "SUCCESS_RATE_PAST_24H",
            Self::AvgJobsPerHourPast24h => "AVG_JOBS_PER_HOUR_PAST_24H",
            Self::AvgJobsPerDayPast7d => "AVG_JOBS_PER_DAY_PAST_7D",
            Self::MostRecentJob => "MOST_RECENT_JOB",
            Self::OldestPendingJob => "OLDEST_PENDING_JOB",
            Self::PeakHourJobs => "PEAK_HOUR_JOBS",
            Self::DbPageSize => "DB_PAGE_SIZE",
            Self::DbPageCount => "DB_PAGE_COUNT",
            Self::DbSize => "DB_SIZE",
        }
    }

    /// Returns the statistic type
    pub fn stat_type(&self) -> StatType {
        match self {
            Self::FailureRate | Self::SuccessRate | Self::SuccessRatePast24h => {
                StatType::Percentage
            }
            Self::AvgJobsPerMinutePastHour
            | Self::AvgJobDurationMins
            | Self::LongestRunningJobMins
            | Self::AvgJobsPerHourPast24h
            | Self::AvgJobsPerDayPast7d => StatType::Decimal,
            Self::MostRecentJob | Self::OldestPendingJob => StatType::Timestamp,
            _ => StatType::Number,
        }
    }

    pub fn translate(&self, i18n: I18nContext<Locale, I18nKeys>) -> AnyView {
        match self {
            Self::PendingJobs => t!(i18n, PENDING_JOBS).into_any(),
            Self::RunningJobs => t!(i18n, RUNNING_JOBS).into_any(),
            Self::FailedJobs => t!(i18n, FAILED_JOBS).into_any(),
            Self::ActiveJobs => t!(i18n, ACTIVE_JOBS).into_any(),
            Self::StaleRunningJobs => t!(i18n, STALE_RUNNING_JOBS).into_any(),
            Self::FailureRate => t!(i18n, FAILURE_RATE).into_any(),
            Self::JobsPastHour => t!(i18n, JOBS_PAST_HOUR).into_any(),
            Self::JobsToday => t!(i18n, JOBS_TODAY).into_any(),
            Self::FailedJobsToday => t!(i18n, FAILED_JOBS_TODAY).into_any(),
            Self::AvgJobsPerMinutePastHour => t!(i18n, AVG_JOBS_PER_MINUTE_PAST_HOUR).into_any(),
            Self::TotalJobs => t!(i18n, TOTAL_JOBS).into_any(),
            Self::DoneJobs => t!(i18n, DONE_JOBS).into_any(),
            Self::CompletedJobs => t!(i18n, COMPLETED_JOBS).into_any(),
            Self::KilledJobs => t!(i18n, KILLED_JOBS).into_any(),
            Self::SuccessRate => t!(i18n, SUCCESS_RATE).into_any(),
            Self::AvgJobDurationMins => t!(i18n, AVG_JOB_DURATION_MINS).into_any(),
            Self::LongestRunningJobMins => t!(i18n, LONGEST_RUNNING_JOB_MINS).into_any(),
            Self::QueueBacklog => t!(i18n, QUEUE_BACKLOG).into_any(),
            Self::JobsPast24Hours => t!(i18n, JOBS_PAST_24_HOURS).into_any(),
            Self::JobsPast7Days => t!(i18n, JOBS_PAST_7_DAYS).into_any(),
            Self::FailedJobsPast7Days => t!(i18n, FAILED_JOBS_PAST_7_DAYS).into_any(),
            Self::SuccessRatePast24h => t!(i18n, SUCCESS_RATE_PAST_24H).into_any(),
            Self::AvgJobsPerHourPast24h => t!(i18n, AVG_JOBS_PER_HOUR_PAST_24H).into_any(),
            Self::AvgJobsPerDayPast7d => t!(i18n, AVG_JOBS_PER_DAY_PAST_7D).into_any(),
            Self::MostRecentJob => t!(i18n, MOST_RECENT_JOB).into_any(),
            Self::OldestPendingJob => t!(i18n, OLDEST_PENDING_JOB).into_any(),
            Self::PeakHourJobs => t!(i18n, PEAK_HOUR_JOBS).into_any(),
            Self::DbPageSize => t!(i18n, DB_PAGE_SIZE).into_any(),
            Self::DbPageCount => t!(i18n, DB_PAGE_COUNT).into_any(),
            Self::DbSize => t!(i18n, DB_SIZE).into_any(),
            _ => self.as_str().into_view().into_any(),
        }
    }
}

impl FromStr for KnownStatistic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RUNNING_JOBS" => Ok(Self::RunningJobs),
            "PENDING_JOBS" => Ok(Self::PendingJobs),
            "FAILED_JOBS" => Ok(Self::FailedJobs),
            "ACTIVE_JOBS" => Ok(Self::ActiveJobs),
            "STALE_RUNNING_JOBS" => Ok(Self::StaleRunningJobs),
            "FAILURE_RATE" => Ok(Self::FailureRate),
            "JOBS_PAST_HOUR" => Ok(Self::JobsPastHour),
            "JOBS_TODAY" => Ok(Self::JobsToday),
            "FAILED_JOBS_TODAY" => Ok(Self::FailedJobsToday),
            "AVG_JOBS_PER_MINUTE_PAST_HOUR" => Ok(Self::AvgJobsPerMinutePastHour),
            "TOTAL_JOBS" => Ok(Self::TotalJobs),
            "DONE_JOBS" => Ok(Self::DoneJobs),
            "COMPLETED_JOBS" => Ok(Self::CompletedJobs),
            "KILLED_JOBS" => Ok(Self::KilledJobs),
            "SUCCESS_RATE" => Ok(Self::SuccessRate),
            "AVG_JOB_DURATION_MINS" => Ok(Self::AvgJobDurationMins),
            "LONGEST_RUNNING_JOB_MINS" => Ok(Self::LongestRunningJobMins),
            "QUEUE_BACKLOG" => Ok(Self::QueueBacklog),
            "JOBS_PAST_24_HOURS" => Ok(Self::JobsPast24Hours),
            "JOBS_PAST_7_DAYS" => Ok(Self::JobsPast7Days),
            "FAILED_JOBS_PAST_7_DAYS" => Ok(Self::FailedJobsPast7Days),
            "SUCCESS_RATE_PAST_24H" => Ok(Self::SuccessRatePast24h),
            "AVG_JOBS_PER_HOUR_PAST_24H" => Ok(Self::AvgJobsPerHourPast24h),
            "AVG_JOBS_PER_DAY_PAST_7D" => Ok(Self::AvgJobsPerDayPast7d),
            "MOST_RECENT_JOB" => Ok(Self::MostRecentJob),
            "OLDEST_PENDING_JOB" => Ok(Self::OldestPendingJob),
            "PEAK_HOUR_JOBS" => Ok(Self::PeakHourJobs),
            "DB_PAGE_SIZE" => Ok(Self::DbPageSize),
            "DB_PAGE_COUNT" => Ok(Self::DbPageCount),
            "DB_SIZE" => Ok(Self::DbSize),
            _ => Err(format!("Unknown statistic: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatType {
    Number,
    Percentage,
    Decimal,
    Timestamp,
}

impl StatType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Number => "Number",
            Self::Percentage => "Percentage",
            Self::Decimal => "Decimal",
            Self::Timestamp => "Timestamp",
        }
    }
}

impl fmt::Display for KnownStatistic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for StatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let stat = KnownStatistic::RunningJobs;
        assert_eq!(KnownStatistic::from_str(stat.as_str()), Some(stat));
    }

    #[test]
    fn test_priority() {
        assert_eq!(KnownStatistic::RunningJobs.priority(), 1);
        assert_eq!(KnownStatistic::ActiveJobs.priority(), 2);
        assert_eq!(KnownStatistic::DbSize.priority(), 9);
    }

    #[test]
    fn test_stat_type() {
        assert_eq!(KnownStatistic::FailureRate.stat_type(), StatType::Percentage);
        assert_eq!(
            KnownStatistic::AvgJobDurationMins.stat_type(),
            StatType::Decimal
        );
        assert_eq!(KnownStatistic::MostRecentJob.stat_type(), StatType::Timestamp);
        assert_eq!(KnownStatistic::RunningJobs.stat_type(), StatType::Number);
    }
}
