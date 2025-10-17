use apalis_board_types::LogEntry;
use futures::{
    Stream, StreamExt,
    channel::mpsc::{Receiver, Sender, TryRecvError, TrySendError, channel},
};
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

#[derive(Debug)]
pub struct Broadcaster {
    clients: Vec<Sender<LogEntry>>,
}

impl Default for Broadcaster {
    fn default() -> Self {
        Self::new()
    }
}

impl Broadcaster {
    pub fn create() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Broadcaster::new()))
    }

    pub fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
        }
    }

    pub fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);

        self.clients.push(tx);
        Client(rx)
    }

    pub fn send(&mut self, msg: LogEntry) -> Result<(), TrySendError<LogEntry>> {
        for client in self.clients.iter_mut().filter(|client| !client.is_closed()) {
            client.try_send(msg.clone())?;
        }
        Ok(())
    }
}

// wrap Receiver in own type, with correct error type
pub struct Client(Receiver<LogEntry>);

impl Stream for Client {
    type Item = Result<LogEntry, TryRecvError>;

    fn poll_next(mut self: Pin<&mut Client>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_next_unpin(cx).map(|c| Ok(c).transpose())
    }
}
