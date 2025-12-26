use apalis_board_types::LogEntry;
use futures::channel::mpsc::{SendError, Sender, channel};
use std::sync::{Arc, Mutex};

use crate::sse::Client;

/// A broadcaster that sends log entries to multiple connected SSE clients.
#[derive(Debug)]
pub struct TracingBroadcaster {
    clients: Vec<Sender<LogEntry>>,
}

impl Default for TracingBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

impl TracingBroadcaster {
    /// Create a new `TracingBroadcaster` wrapped in an `Arc<Mutex<>>`.
    #[must_use]
    pub fn create() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::new()))
    }

    /// Create a new `TracingBroadcaster`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
        }
    }

    /// Create a new client and register it with the broadcaster.
    pub fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);

        self.clients.push(tx);
        Client(rx)
    }

    /// Send a log entry to all connected clients.
    pub fn send(&mut self, msg: &LogEntry) -> Result<(), SendError> {
        for client in self.clients.iter_mut().filter(|client| !client.is_closed()) {
            client
                .try_send(msg.clone())
                .map_err(|e| e.into_send_error())?;
        }
        Ok(())
    }
}
