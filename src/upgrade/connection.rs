use async_std::io::{self, prelude::*};

use std::pin::Pin;
use std::task::{Context, Poll};

/// An upgraded HTTP connection.
#[derive(Debug, Clone)]
pub struct Connection {
    inner: Pin<Box<dyn Read + Write + Clone + Send + Sync + Unpin + 'static>>,
}

impl Read for Connection {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.project();
        Pin::new(this.inner).poll_read(cx, buf)
    }
}

impl Write for Connection {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.project();
        Pin::new(this.inner).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.project();
        Pin::new(this.inner).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.project();
        Pin::new(this.inner).poll_close(cx)
    }
}
