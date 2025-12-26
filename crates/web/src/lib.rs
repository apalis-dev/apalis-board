#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use apalis_board_types::LogEntry;
use chrono::{DateTime, Local, Utc};
use leptos::{prelude::*, reactive::spawn_local};
use serde::{Deserialize, Serialize};

pub mod api;
pub mod components;
pub mod pages;
pub mod translate;

#[allow(deprecated)]
pub mod locales {
    // TODO: @geofmureithi move to build script
    leptos_i18n::load_locales!();
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
}

type UserSignal = RwSignal<Option<User>>;
pub fn use_user_signal() -> UserSignal {
    use_context::<UserSignal>().expect("UserSignal")
}

pub fn use_sse_signal() -> RwSignal<Vec<String>> {
    use_context::<RwSignal<Vec<String>>>().expect("SSE Signal")
}

#[derive(Clone, Copy)]
pub struct SseProvider {
    event_source: RwSignal<LogEntry>,
    is_healthy: RwSignal<bool>,
}

impl SseProvider {
    pub fn event_source(&self) -> RwSignal<LogEntry> {
        self.event_source
    }

    pub fn is_healthy(&self) -> RwSignal<bool> {
        self.is_healthy
    }
}

pub fn use_sse_provider() -> SseProvider {
    use_context::<SseProvider>().expect("SSE Provider")
}

pub fn create_sse_resource(url: &str) -> SseProvider {
    use futures::StreamExt;
    let data = RwSignal::new(LogEntry::default());
    let mut source = gloo_net::eventsource::futures::EventSource::new(url)
        .expect("couldn't connect to SSE stream");
    let mut stream = source
        .subscribe("message")
        .unwrap()
        .map(|value| match value {
            Ok(value) => Ok(serde_json::from_str::<LogEntry>(
                &value.1.data().as_string().expect("expected string value"),
            )
            .expect("couldn't parse message")),
            Err(e) => Err(e),
        });
    let is_healthy = RwSignal::new(true);
    spawn_local(async move {
        while let Some(next_value) = stream.next().await {
            if let Ok(log_entry) = next_value {
                data.set(log_entry);
            } else {
                is_healthy.set(false);
                source.close();
                break;
            }
        }
    });
    SseProvider {
        event_source: data,
        is_healthy,
    }
}

pub fn relative_timestamp(timestamp: u64) -> String {
    let now = Utc::now().timestamp() as u64;

    match timestamp.cmp(&now) {
        std::cmp::Ordering::Greater => {
            let future_diff = timestamp - now;
            match future_diff {
                0..=59 => "in a few seconds".to_string(),
                60..=3599 => format!("in {} minutes", future_diff / 60),
                3600..=86399 => format!("in {} hours", future_diff / 3600),
                86400..=2_592_000 => format!("in {} days", future_diff / 86400),
                _ => {
                    let datetime = DateTime::<Utc>::from_timestamp(timestamp as i64, 0)
                        .expect("Invalid timestamp");
                    let local: DateTime<Local> = datetime.into();
                    format!("on {}", local.format("%B %d, %Y"))
                }
            }
        }
        _ => {
            let diff = now.saturating_sub(timestamp);
            match diff {
                0..=59 => "just now".to_string(),
                60..=3599 => format!("{} minutes ago", diff / 60),
                3600..=86399 => format!("{} hours ago", diff / 3600),
                86400..=2_592_000 => format!("{} days ago", diff / 86400),
                _ => {
                    let datetime = DateTime::<Utc>::from_timestamp(timestamp as i64, 0)
                        .expect("Invalid timestamp");
                    let local: DateTime<Local> = datetime.into();
                    local.format("%B %d, %Y").to_string()
                }
            }
        }
    }
}

pub type RawTask = apalis_core::task::Task<serde_json::Value, serde_json::Value, String>;
