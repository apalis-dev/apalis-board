#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../README.md"))]
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    #[default]
    Debug,
    Trace,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogEntry {
    pub span: Option<Span>,
    pub level: LogLevel,
    pub timestamp: String,
    pub target: String,
    #[serde(rename = "fields")]
    pub entry: Entry,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineEntry {
    #[serde(rename = "log.target")]
    pub log_target: String,
    #[serde(rename = "log.module_path")]
    pub log_module_path: String,
    #[serde(rename = "log.file")]
    pub log_file: String,
    #[serde(rename = "log.line")]
    pub log_line: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EntryType {
    Result(ResultEntry),
    Line(LineEntry),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct ResultEntry {
    pub done_in: String,
    pub result: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Entry {
    pub message: Option<String>,
    #[serde(flatten)]
    pub entry_type: Option<EntryType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Span {
    pub attempt: i64,
    #[serde(rename = "task_id")]
    pub task_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "message")]
pub enum ApiError {
    CodecError(String),
    BackendError(String),
}
