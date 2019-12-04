use async_std::io::{self, BufRead, Read};

use std::borrow::Borrow;
use std::fmt::{self, Debug};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::headers::{self, HeaderName, HeaderValue, Headers};
use crate::mime::{self, Mime};
use crate::{Method, Url};

type BodyReader = dyn BufRead + Unpin + Send + 'static;

pin_project_lite::pin_project! {
    /// An HTTP request.
    pub struct Request {
        method: Method,
        url: Url,
        headers: Headers,
        #[pin]
        body_reader: Box<BodyReader>,
        length: Option<usize>,
    }
}

impl Request {
    /// Create a new request.
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: Headers::new(),
            body_reader: Box::new(io::empty()),
            length: None,
        }
    }

    /// Get the HTTP method
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Get the url
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get the headers
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Get the body
    pub fn body_reader(&self) -> &Box<BodyReader> {
        &self.body_reader
    }

    /// Consume self and get body
    pub fn into_body_reader(self) -> Box<BodyReader> {
        self.body_reader
    }

    /// Set the body reader.
    pub fn set_body_reader(mut self, body: impl BufRead + Unpin + Send + 'static) -> Self {
        self.body_reader = Box::new(body);
        self
    }

    /// Set the lengths of the body.
    pub fn set_length(mut self, length: usize) -> Self {
        self.length = Some(length);
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

    /// Get an HTTP header..
    pub fn header(&self, name: impl Borrow<HeaderName>) -> Option<&HeaderValue> {
        self.headers.get(name.borrow())
    }

    /// Set an HTTP header.
    pub fn set_header(
        &mut self,
        name: HeaderName,
        value: HeaderValue,
    ) -> io::Result<Option<HeaderValue>> {
        Ok(self.headers.insert(name, value))
    }

    /// Set the response MIME.
    pub fn set_mime(self, mime: Mime) -> io::Result<Self> {
        let name = HeaderName::new("content-type".to_string().as_ref());
        self.set_header(name, mime.into())
    }

    /// Get the length of the body stream, if it has been set.
    ///
    /// This value is set when passing a fixed-size object into as the body. E.g. a string, or a
    /// buffer. Consumers of this API should check this value to decide whether to use `Chunked`
    /// encoding, or set the response length.
    pub fn len(&self) -> Option<usize> {
        self.length
    }

    /// Set the length of the body stream
    pub fn set_len(mut self, len: usize) -> Self {
        self.length = Some(len);
        self
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

impl Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.method)
            .field("url", &self.url)
            .field("headers", &self.headers)
            .field("body", &"<hidden>")
            .finish()
    }
}

impl Read for Request {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.body_reader).poll_read(cx, buf)
    }
}

impl BufRead for Request {
    #[allow(missing_doc_code_examples)]
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&'_ [u8]>> {
        let this = self.project();
        this.body_reader.poll_fill_buf(cx)
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        Pin::new(&mut self.body_reader).consume(amt)
    }
}

impl AsRef<Headers> for Request {
    fn as_ref(&self) -> &Headers {
        &self.headers
    }
}

impl AsMut<Headers> for Request {
    fn as_mut(&mut self) -> &mut Headers {
        &mut self.headers
    }
}

impl IntoIterator for Request {
    type Item = (HeaderName, HeaderValue);
    type IntoIter = headers::IntoIter;

    /// Returns a iterator of references over the remaining items.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.into_iter()
    }
}

impl<'a> IntoIterator for &'a Request {
    type Item = (&'a HeaderName, &'a HeaderValue);
    type IntoIter = headers::Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.iter()
    }
}

impl<'a> IntoIterator for &'a mut Request {
    type Item = (&'a HeaderName, &'a mut HeaderValue);
    type IntoIter = headers::IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.iter_mut()
    }
}
