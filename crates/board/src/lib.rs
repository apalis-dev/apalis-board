use apalis_board_types::LogEntry;
use futures::StreamExt;
use leptos::{prelude::*, reactive::spawn_local};
use serde::{Deserialize, Serialize};

pub mod api;
pub mod components;
pub mod pages;
pub mod translate;

leptos_i18n::load_locales!();

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

#[derive(Clone)]
pub struct SseProvider {
    event_source: RwSignal<LogEntry>,
}

impl SseProvider {
    pub fn event_source(&self) -> RwSignal<LogEntry> {
        self.event_source
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
    spawn_local(async move {
        while let Some(next_value) = stream.next().await {
            if let Ok(log_entry) = next_value {
                data.set(log_entry);
            }
        }
    });

    std::mem::forget(source);
    SseProvider { event_source: data }
}

pub type RawTask = apalis_core::task::Task<serde_json::Value, serde_json::Value, String>;
