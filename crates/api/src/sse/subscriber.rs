use std::sync::Arc;
use std::{io::LineWriter, sync::Mutex};

use tracing_subscriber::fmt::format::{Format, Json, JsonFields};
use tracing_subscriber::fmt::{Layer, MakeWriter};

use tracing_subscriber::registry::LookupSpan;

use crate::sse::broadcaster::TracingBroadcaster;

#[derive(Debug, Clone)]
pub struct TracingSubscriber {
    broadcaster: Arc<Mutex<TracingBroadcaster>>,
}

impl TracingSubscriber {
    pub fn new(broadcaster: &Arc<Mutex<TracingBroadcaster>>) -> Self {
        TracingSubscriber {
            broadcaster: broadcaster.clone(),
        }
    }

    pub fn new_raw(broadcaster: TracingBroadcaster) -> Self {
        TracingSubscriber {
            broadcaster: Arc::new(Mutex::new(broadcaster)),
        }
    }

    pub fn get_broadcaster(&self) -> Arc<Mutex<TracingBroadcaster>> {
        self.broadcaster.clone()
    }

    pub fn layer<S>(self) -> Layer<S, JsonFields, Format<Json>, TracingSubscriber>
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
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("JSON Error: {e}"),
            )
        })?;
        self.broadcaster
            .try_lock()
            .map(|mut b| {
                b.send(log_entry).map_err(|e| {
                    std::io::Error::other(
                        format!("Broadcast Error: {e}"),
                    )
                })
            })
            .map_err(|e| {
                std::io::Error::other(format!("Lock Error: {e}"))
            })??;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
