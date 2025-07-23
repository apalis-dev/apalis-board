use std::sync::Arc;
use std::{io::LineWriter, sync::Mutex};

use apalis_board_utils::sse::Broadcaster;
use tracing_subscriber::fmt::MakeWriter;

#[derive(Debug, Clone)]
pub struct Subscriber {
    pub tx: Arc<Mutex<Broadcaster>>,
}

impl<'a> MakeWriter<'a> for Subscriber {
    type Writer = LineWriter<Self>;

    fn make_writer(&self) -> Self::Writer {
        LineWriter::new(Self {
            tx: self.tx.clone(),
        })
    }
}

impl std::io::Write for Subscriber {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        let _ = self
            .tx
            .try_lock()
            .map(|b| b.send(std::str::from_utf8(buf).unwrap_or_default()));
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
