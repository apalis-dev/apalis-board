use std::sync::Arc;
use std::{io::LineWriter, sync::Mutex};

use tracing_subscriber::fmt::format::{Format, Json, JsonFields};
use tracing_subscriber::fmt::{Layer, MakeWriter};

use tracing_subscriber::registry::LookupSpan;

use crate::sse::broadcaster::TracingBroadcaster;

/// A tracing subscriber that sends log entries to a `TracingBroadcaster`.
#[derive(Debug, Clone)]
pub struct TracingSubscriber {
    broadcaster: Arc<Mutex<TracingBroadcaster>>,
}

impl TracingSubscriber {
    /// Create a new `TracingSubscriber` from a broadcaster reference.
    pub fn new(broadcaster: &Arc<Mutex<TracingBroadcaster>>) -> Self {
        Self {
            broadcaster: broadcaster.clone(),
        }
    }

    /// Create a new `TracingSubscriber` from a raw broadcaster.
    #[must_use] 
    pub fn new_inner(broadcaster: TracingBroadcaster) -> Self {
        Self {
            broadcaster: Arc::new(Mutex::new(broadcaster)),
        }
    }

    /// Get the underlying broadcaster.
    #[must_use] 
    pub fn get_broadcaster(&self) -> &Arc<Mutex<TracingBroadcaster>> {
        &self.broadcaster
    }

    /// Create a tracing layer that uses this subscriber as a writer.
    #[must_use] 
    pub fn layer<S>(self) -> Layer<S, JsonFields, Format<Json>, Self>
    where
        S: tracing_core::Subscriber + for<'a> LookupSpan<'a>,
    {
        tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .fmt_fields(JsonFields::new())
            .event_format(tracing_subscriber::fmt::format().with_ansi(false).json())
            .with_writer(self)
    }
}

impl<'a> MakeWriter<'a> for TracingSubscriber {
    type Writer = LineWriter<Self>;

    fn make_writer(&self) -> Self::Writer {
        LineWriter::new(Self {
            broadcaster: self.broadcaster.clone(),
        })
    }
}

impl std::io::Write for TracingSubscriber {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        let msg = std::str::from_utf8(buf).unwrap_or_default();
        let log_entry = serde_json::from_str(msg).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("JSON Error: {e}"))
        })?;
        self.broadcaster
            .try_lock()
            .map(|mut b| {
                b.send(&log_entry)
                    .map_err(|e| std::io::Error::other(format!("Broadcast Error: {e}")))
            })
            .map_err(|e| std::io::Error::other(format!("Lock Error: {e}")))??;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
