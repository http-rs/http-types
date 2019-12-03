use async_std::io::{self, BufRead, Read};

use std::borrow::Borrow;
use std::fmt::{self, Debug};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::mime::{self, Mime};
use crate::{headers, Headers, Method, Url};

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

    /// Create a new GET request.
    pub fn get(url: Url) -> Self {
        Self::new(Method::Get, url)
    }

    /// Create a new HEAD request.
    pub fn head(url: Url) -> Self {
        Self::new(Method::Head, url)
    }

    /// Create a new POST request.
    pub fn post(url: Url) -> Self {
        Self::new(Method::Post, url)
    }

    /// Create a new PUT request.
    pub fn put(url: Url) -> Self {
        Self::new(Method::Put, url)
    }

    /// Create a new DELETE request.
    pub fn delete(url: Url) -> Self {
        Self::new(Method::Delete, url)
    }

    /// Create a new CONNECT request.
    pub fn connect(url: Url) -> Self {
        Self::new(Method::Connect, url)
    }

    /// Create a new OPTIONS request.
    pub fn options(url: Url) -> Self {
        Self::new(Method::Options, url)
    }

    /// Create a new TRACE request.
    pub fn trace(url: Url) -> Self {
        Self::new(Method::Trace, url)
    }

    /// Create a new PATCH request.
    pub fn patch(url: Url) -> Self {
        Self::new(Method::Patch, url)
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
    type Item = (String, String);
    type IntoIter = headers::IntoIter;

    /// Returns a iterator of references over the remaining items.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.into_iter()
    }
}

impl<'a> IntoIterator for &'a Request {
    type Item = (&'a String, &'a String);
    type IntoIter = headers::Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.iter()
    }
}

impl<'a> IntoIterator for &'a mut Request {
    type Item = (&'a String, &'a mut String);
    type IntoIter = headers::IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.iter_mut()
    }
}
