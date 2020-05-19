use async_std::io::{self, prelude::*};

use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};

/// An upgraded HTTP connection.
pub struct Connection {
    inner: Pin<Box<dyn InnerConnection + 'static>>,
}

pub(crate) trait InnerConnection: Read + Write + Send + Sync + Unpin {}
impl<T: Read + Write + Send + Sync + Unpin> InnerConnection for T {}

impl Read for Connection {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl Write for Connection {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_close(cx)
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection")
            .field("inner", &"Pin<Box<dyn Inner>>")
            .finish()
    }
}
