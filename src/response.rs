use async_std::io::{self, BufRead, Read};

use std::borrow::Borrow;
use std::convert::TryInto;
use std::fmt::{self, Debug};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::mime::{self, Mime};
use crate::{Headers, StatusCode};

type BodyReader = dyn BufRead + Unpin + Send + 'static;

pin_project_lite::pin_project! {
    /// An HTTP response.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #
    /// use http_types::{Response, StatusCode};
    ///
    /// let mut req = Response::new(200)?;
    /// #
    /// # Ok(()) }
    /// ```
    pub struct Response {
        #[pin]
        body_reader: Box<BodyReader>,
        status: StatusCode,
        headers: Headers,
        length: Option<usize>,
    }
}

impl Response {
    /// Create a new response.
    pub fn new<S>(status: S) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>>
    where
        S: TryInto<StatusCode>,
        <S as TryInto<StatusCode>>::Error: Sync + Send + std::error::Error + 'static,
    {
        Ok(Self {
            status: status.try_into()?,
            headers: Headers::new(),
            body_reader: Box::new(io::empty()),
            length: Some(0),
        })
    }

    /// Get the status
    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    /// Set the body reader to `body` and unset the length.
    ///
    /// This will make the body a chunked stream.
    pub fn set_body_reader(mut self, body: impl BufRead + Unpin + Send + 'static) -> Self {
        self.body_reader = Box::new(body);
        self.length = None;
        self
    }

    /// Set the body as a string.
    ///
    /// # Mime
    ///
    /// The encoding is set to `text/plain; charset=utf-8`.
    pub fn set_body_string(mut self, string: String) -> io::Result<Self> {
        self.length = Some(string.len());
        let reader = io::Cursor::new(string.into_bytes());
        self.set_body_reader(reader).set_mime(mime::PLAIN)
    }

    /// Pass bytes as the request body.
    ///
    /// # Mime
    ///
    /// The encoding is set to `application/octet-stream`.
    pub fn set_body_bytes(mut self, bytes: impl AsRef<[u8]>) -> io::Result<Self> {
        let bytes = bytes.as_ref().to_owned();
        self.length = Some(bytes.len());
        let reader = io::Cursor::new(bytes);
        self.set_body_reader(reader).set_mime(mime::BYTE_STREAM)
    }

    /// Get HTTP headers.
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Get an HTTP header.
    pub fn header(&self, key: impl Borrow<str>) -> Option<&String> {
        self.headers.get(key)
    }

    /// Set an HTTP header.
    pub fn set_header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> io::Result<Self> {
        let key = key.as_ref().to_owned();
        let value = value.as_ref().to_owned();
        self.headers.insert(key, value)?; // TODO: this should be a Result because only ASCII values are allowed
        Ok(self)
    }

    /// Set the response MIME.
    pub fn set_mime(self, mime: Mime) -> io::Result<Self> {
        self.set_header("Content-Type", format!("{}", mime))
    }

    /// Get the length of the body stream, if it has been set.
    ///
    /// This value is set when passing a fixed-size object into as the body. E.g. a string, or a
    /// buffer. Consumers of this API should check this value to decide whether to use `Chunked`
    /// encoding, or set the response length.
    pub fn len(&self) -> Option<usize> {
        self.length
    }

    /// Set the length of the body stream.
    pub fn set_len(mut self, len: usize) -> Self {
        self.length = Some(len);
        self
    }
}

impl Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("headers", &self.headers)
            .field("body", &"<hidden>")
            .finish()
    }
}

impl Read for Response {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.body_reader).poll_read(cx, buf)
    }
}

impl BufRead for Response {
    #[allow(missing_doc_code_examples)]
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&'_ [u8]>> {
        let this = self.project();
        this.body_reader.poll_fill_buf(cx)
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        Pin::new(&mut self.body_reader).consume(amt)
    }
}
