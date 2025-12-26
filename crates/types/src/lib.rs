#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
use serde::{Deserialize, Serialize};

/// Configuration module.
pub mod config;

/// The log level of a log entry.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    /// An informational level for general information.
    Info,
    /// A warning level for potential issues.
    Warn,
    /// An error level for serious issues.
    Error,
    /// Debug level for detailed debugging information.
    #[default]
    Debug,
    /// Trace level for detailed tracing information.
    Trace,
}

/// A log entry.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    /// An optional tracing span associated with the log entry.
    pub span: Option<Span>,
    /// The log level of the entry.
    pub level: LogLevel,
    /// The timestamp of the log entry.
    pub timestamp: String,
    /// The target of the log entry.
    pub target: String,
    /// The main content of the log entry.
    #[serde(rename = "fields")]
    pub entry: Entry,
}

/// A line entry.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineEntry {
    /// The log target.
    #[serde(rename = "log.target")]
    pub log_target: String,
    /// The module path of the log.
    #[serde(rename = "log.module_path")]
    pub log_module_path: String,
    /// The file where the log was generated.
    #[serde(rename = "log.file")]
    pub log_file: String,
    /// The line number in the file where the log was generated.
    #[serde(rename = "log.line")]
    pub log_line: i64,
}

/// Additional information about a log entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntryType {
    /// A result entry.
    Result(ResultEntry),
    /// A line entry.
    Line(LineEntry),
}

/// A result entry.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct ResultEntry {
    /// The time taken to complete the task.
    pub done_in: String,
    /// The result of the task.
    pub result: String,
}

/// A log entry.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Entry {
    /// The log message.
    pub message: Option<String>,
    /// Additional entry type information.
    #[serde(flatten)]
    pub entry_type: Option<EntryType>,
}

/// Information about a tracing span.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Span {
    /// The unique identifier for the span.
    pub attempt: i64,
    /// The task ID associated with the span.
    pub task_id: String,
    /// The name of the span.
    pub name: String,
}

/// An enumeration of possible API errors.
#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum ApiError {
    /// Error related to codec operations.
    #[error("Codec error: {0}")]
    CodecError(String),
    /// Error related to backend operations.
    #[error("Backend error: {0}")]
    BackendError(String),
}
