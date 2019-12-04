use async_std::io::{self, BufRead, Read};

use std::fmt::{self, Debug};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::headers::{self, HeaderName, HeaderValue, Headers};
use crate::mime::{self, Mime};
use crate::StatusCode;

type BodyReader = dyn BufRead + Unpin + Send + 'static;

pin_project_lite::pin_project! {
    /// An HTTP response.
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
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: Headers::new(),
            body_reader: Box::new(io::empty()),
            length: Some(0),
        }
    }

    /// Get the status
    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    /// Set the body reader to `body` and unset the length.
    ///
    /// This will make the body a chunked stream.
    pub fn set_body_reader(&mut self, body: impl BufRead + Unpin + Send + 'static) {
        self.body_reader = Box::new(body);
        self.length = None;
    }

    /// Set the body as a string.
    ///
    /// # Mime
    ///
    /// The encoding is set to `text/plain; charset=utf-8`.
    pub fn set_body_string(&mut self, string: String) -> io::Result<()> {
        self.length = Some(string.len());
        let reader = io::Cursor::new(string.into_bytes());
        self.set_body_reader(reader);
        self.set_mime(mime::PLAIN)?;
        Ok(())
    }

    /// Pass bytes as the request body.
    ///
    /// # Mime
    ///
    /// The encoding is set to `application/octet-stream`.
    pub fn set_body_bytes(&mut self, bytes: impl AsRef<[u8]>) -> io::Result<()> {
        let bytes = bytes.as_ref().to_owned();
        self.length = Some(bytes.len());
        let reader = io::Cursor::new(bytes);
        self.set_body_reader(reader);
        self.set_mime(mime::BYTE_STREAM)?;
        Ok(())
    }

    /// Get HTTP headers.
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Get an HTTP header.
    pub fn header(&self, name: &HeaderName) -> Option<&Vec<HeaderValue>> {
        self.headers.get(name)
    }

    /// Set an HTTP header.
    pub fn set_header(
        &mut self,
        name: HeaderName,
        value: HeaderValue,
    ) -> io::Result<Option<Vec<HeaderValue>>> {
        self.headers.insert(name, value)
    }

    /// Set the response MIME.
    pub fn set_mime(&mut self, mime: Mime) -> io::Result<Option<Vec<HeaderValue>>> {
        let header = HeaderName {
            string: "content-type".to_string(),
        };
        self.set_header(header, mime.into())
    }

    /// Get the length of the body stream, if it has been set.
    ///
    /// This value is set when passing a fixed-size object into as the body. E.g. a string, or a
    /// buffer. Consumers of this API should check this value to decide whether to use `Chunked`
    /// encoding, or set the response length.
    pub fn len(&self) -> Option<usize> {
        self.length
    }

    /// An iterator visiting all header pairs in arbitrary order.
    pub fn iter<'a>(&'a self) -> headers::Iter<'a> {
        self.headers.iter()
    }

    /// An iterator visiting all header pairs in arbitrary order, with mutable references to the
    /// values.
    pub fn iter_mut<'a>(&'a mut self) -> headers::IterMut<'a> {
        self.headers.iter_mut()
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

impl AsRef<Headers> for Response {
    fn as_ref(&self) -> &Headers {
        &self.headers
    }
}

impl AsMut<Headers> for Response {
    fn as_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }
}

impl IntoIterator for Response {
    type Item = (HeaderName, Vec<HeaderValue>);
    type IntoIter = headers::IntoIter;

    /// Returns a iterator of references over the remaining items.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.into_iter()
    }
}

impl<'a> IntoIterator for &'a Response {
    type Item = (&'a HeaderName, &'a Vec<HeaderValue>);
    type IntoIter = headers::Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.iter()
    }
}

impl<'a> IntoIterator for &'a mut Response {
    type Item = (&'a HeaderName, &'a mut Vec<HeaderValue>);
    type IntoIter = headers::IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.iter_mut()
    }
}
