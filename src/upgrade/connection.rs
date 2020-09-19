use futures_lite::*;

use std::pin::Pin;
use std::task::{Context, Poll};

/// An upgraded HTTP connection.
#[derive(Debug, Clone)]
pub struct RawConnection<Inner> {
    inner: Inner,
}

/// A boxed upgraded HTTP connection.
pub type Connection = RawConnection<Box<dyn InnerConnection + 'static>>;

/// Trait to signal the requirements for an underlying connection type.
pub trait InnerConnection: AsyncRead + AsyncWrite + Send + Sync + Unpin {}
impl<T: AsyncRead + AsyncWrite + Send + Sync + Unpin> InnerConnection for T {}

impl<Inner: AsyncRead + Unpin> AsyncRead for RawConnection<Inner> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl<Inner: AsyncWrite + Unpin> AsyncWrite for RawConnection<Inner> {
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
