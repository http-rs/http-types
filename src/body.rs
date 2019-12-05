use async_std::io::{self, BufRead, Read};

use std::fmt::{self, Debug};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::{mime, Mime};

pin_project_lite::pin_project! {
    /// A streaming HTTP body.
    pub struct Body {
        #[pin]
        reader: Box<dyn BufRead + Unpin + Send + 'static>,
        mime: Option<Mime>,
        length: Option<usize>,
    }
}

impl Body {
    /// Create a new empty `Body`.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_types::Body;
    ///
    /// let _body = Body::empty();
    /// ```
    pub fn empty() -> Self {
        Self {
            reader: Box::new(io::empty()),
            mime: Some(mime::BYTE_STREAM),
            length: Some(0),
        }
    }

    /// Create a `Body` from a reader.
    pub fn from_reader(reader: impl BufRead + Unpin + Send + 'static) -> Self {
        Self {
            reader: Box::new(reader),
            mime: Some(mime::BYTE_STREAM),
            length: None,
        }
    }

    /// Get the recommended mime type.
    pub(crate) fn take_mime(&mut self) -> Mime {
        self.mime.take().unwrap()
    }

    /// Get the length of the body in bytes.
    pub fn len(&self) -> Option<usize> {
        self.length
    }

    /// Get the length of the body in bytes.
    pub fn set_len(&mut self, length: usize) {
        self.length = Some(length);
    }
}

impl Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Body")
            .field("reader", &"<hidden>")
            .finish()
    }
}

impl From<String> for Body {
    fn from(s: String) -> Self {
        Self {
            length: Some(s.len()),
            reader: Box::new(io::Cursor::new(s.into_bytes())),
            mime: Some(mime::PLAIN),
        }
    }
}

impl Read for Body {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.reader).poll_read(cx, buf)
    }
}

impl BufRead for Body {
    #[allow(missing_doc_code_examples)]
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&'_ [u8]>> {
        let this = self.project();
        this.reader.poll_fill_buf(cx)
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        Pin::new(&mut self.reader).consume(amt)
    }
}
