use std::sync::Arc;
use std::{io::LineWriter, sync::Mutex};

use crate::sse::Broadcaster;
use tracing_subscriber::fmt::MakeWriter;

#[derive(Debug, Clone)]
pub struct Subscriber {
    broadcaster: Arc<Mutex<Broadcaster>>,
}

impl Subscriber {
    pub fn new(broadcaster: &Arc<Mutex<Broadcaster>>) -> Self {
        Subscriber {
            broadcaster: broadcaster.clone(),
        }
    }

    pub fn new_raw(broadcaster: Broadcaster) -> Self {
        Subscriber {
            broadcaster: Arc::new(Mutex::new(broadcaster)),
        }
    }

    pub fn get_broadcaster(&self) -> Arc<Mutex<Broadcaster>> {
        self.broadcaster.clone()
    }
}

impl<'a> MakeWriter<'a> for Subscriber {
    type Writer = LineWriter<Self>;

    fn make_writer(&self) -> Self::Writer {
        LineWriter::new(Self {
            broadcaster: self.broadcaster.clone(),
        })
    }
}

impl std::io::Write for Subscriber {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        let msg = std::str::from_utf8(buf).unwrap_or_default();
        let log_entry = serde_json::from_str(msg).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("JSON Error: {}", e),
            )
        })?;
        self.broadcaster
            .try_lock()
            .map(|mut b| {
                b.send(log_entry).map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Broadcast Error: {}", e),
                    )
                })
            })
            .map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::Other, format!("Lock Error: {}", e))
            })??;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
