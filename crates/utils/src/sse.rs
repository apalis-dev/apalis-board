use bytes::Bytes;
use futures::{
    channel::mpsc::{channel, Receiver, Sender, TryRecvError},
    Stream, StreamExt,
};
use std::{
    pin::Pin,
    sync::Arc,
    sync::Mutex,
    task::{Context, Poll},
};

#[derive(Debug)]
pub struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}

impl Default for Broadcaster {
    fn default() -> Self {
        Self::new()
    }
}

impl Broadcaster {
    pub fn create() -> Arc<Mutex<Self>> {
        let me = Arc::new(Mutex::new(Broadcaster::new()));
        me
    }

    pub fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
        }
    }

    pub fn remove_stale_clients(&mut self) {
        let mut ok_clients = Vec::new();
        for client in self.clients.iter() {
            let result = client.clone().try_send(Bytes::from("data: ping\n\n"));

            if let Ok(()) = result {
                ok_clients.push(client.clone());
            }
        }
        self.clients = ok_clients;
    }

    pub fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(5);

        tx.clone()
            .try_send(Bytes::from("data: connected\n\n"))
            .unwrap();

        self.clients.push(tx);
        Client(rx)
    }

    pub fn send(&self, msg: &str) {
        let msg = Bytes::from(["data: ", &msg, "\n\n"].concat());

        for client in self.clients.iter().filter(|client| !client.is_closed()) {
            client.clone().try_send(msg.clone()).unwrap();
        }
    }
}

// wrap Receiver in own type, with correct error type
pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, TryRecvError>;

    fn poll_next(mut self: Pin<&mut Client>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_next_unpin(cx).map(|c| Ok(c).transpose())
    }
}
