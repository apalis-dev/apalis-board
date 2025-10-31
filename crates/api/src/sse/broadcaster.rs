use apalis_board_types::LogEntry;
use futures::channel::mpsc::{SendError, Sender, channel};
use std::sync::{Arc, Mutex};

use crate::sse::Client;

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
    pub fn create() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(TracingBroadcaster::new()))
    }

    pub fn new() -> Self {
        TracingBroadcaster {
            clients: Vec::new(),
        }
    }

    pub fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);

        self.clients.push(tx);
        Client(rx)
    }

    pub fn send(&mut self, msg: LogEntry) -> Result<(), SendError> {
        for client in self.clients.iter_mut().filter(|client| !client.is_closed()) {
            client
                .try_send(msg.clone())
                .map_err(|e| e.into_send_error())?;
        }
        Ok(())
    }
}
