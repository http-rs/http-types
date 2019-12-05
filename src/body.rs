use async_std::io::{self, BufRead, Read};

use std::fmt::{self, Debug};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::{mime, Mime};

pin_project_lite::pin_project! {
    /// A streaming HTTP body.
    pub struct Body {
        #[pin]
        body_reader: Box<dyn BufRead + Unpin + Send + 'static>,
        buf: Option<Vec<u8>>,
        mime: Mime,
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
            body_reader: Box::new(io::empty()),
            buf: None,
            mime: mime::BYTE_STREAM,
            length: Some(0),
        }
    }

    /// Create a `Body` from a reader.
    pub fn from_reader(reader: impl BufRead + Unpin + Send + 'static) -> Self {
        Self {
            body_reader: Box::new(reader),
            buf: None,
            mime: mime::BYTE_STREAM,
            length: None,
        }
    }

    /// Get the recommended mime type.
    pub(crate) fn mime(&self) -> &Mime {
        &self.mime
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
            .field("body_reader", &"<hidden>")
            .finish()
    }
}

impl Read for Body {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.body_reader).poll_read(cx, buf)
    }
}

impl BufRead for Body {
    #[allow(missing_doc_code_examples)]
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&'_ [u8]>> {
        let this = self.project();
        this.body_reader.poll_fill_buf(cx)
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        Pin::new(&mut self.body_reader).consume(amt)
    }
}
