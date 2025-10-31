use std::{
    pin::Pin,
    task::{Context, Poll},
};

use apalis_board_types::LogEntry;
use futures::{
    Stream, StreamExt,
    channel::mpsc::{Receiver, TryRecvError},
};

// wrap Receiver in own type, with correct error type
pub struct Client(pub(crate) Receiver<LogEntry>);

impl Stream for Client {
    type Item = Result<LogEntry, TryRecvError>;

    fn poll_next(mut self: Pin<&mut Client>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_next_unpin(cx).map(|c| Ok(c).transpose())
    }
}
