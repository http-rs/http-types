use async_std::io::{self, BufRead, Read};

use std::pin::Pin;
use std::task::{Context, Poll};

use crate::headers::{self, HeaderName, HeaderValue, Headers, Names, ToHeaderValues, Values};
use crate::mime::Mime;
use crate::{Body, StatusCode};

pin_project_lite::pin_project! {
    /// An HTTP response.
    #[derive(Debug)]
    pub struct Response {
        status: StatusCode,
        headers: Headers,
        #[pin]
        body: Body,
    }
}

impl Response {
    /// Create a new response.
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: Headers::new(),
            body: Body::empty(),
        }
    }

    /// Get the status
    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    /// Get a mutable reference to a header.
    pub fn header_mut(&mut self, name: &HeaderName) -> Option<&mut Vec<HeaderValue>> {
        self.headers.get_mut(name)
    }

    /// Get an HTTP header.
    pub fn header(&self, name: &HeaderName) -> Option<&Vec<HeaderValue>> {
        self.headers.get(name)
    }

    /// Set an HTTP header.
    pub fn insert_header(
        &mut self,
        name: HeaderName,
        values: impl ToHeaderValues,
    ) -> io::Result<Option<Vec<HeaderValue>>> {
        self.headers.insert(name, values)
    }

    /// Append a header to the headers.
    ///
    /// Unlike `insert` this function will not override the contents of a header, but insert a
    /// header if there aren't any. Or else append to the existing list of headers.
    pub fn append_header(
        &mut self,
        name: HeaderName,
        values: impl ToHeaderValues,
    ) -> io::Result<()> {
        self.headers.append(name, values)
    }

    /// Get the body.
    pub fn body(&self) -> &Body {
        &self.body
    }

    /// Set the body reader.
    pub fn set_body(&mut self, body: impl Into<Body>) {
        self.body = body.into();
        let mime = self.body.take_mime();
        self.set_content_type(mime);
    }

    /// Set the response MIME.
    pub fn set_content_type(&mut self, mime: Mime) -> Option<Vec<HeaderValue>> {
        let header = HeaderName {
            string: String::new(),
            static_str: Some("content-type"),
        };
        let value: HeaderValue = mime.into();

        // A Mime instance is guaranteed to be valid header name.
        self.insert_header(header, value).unwrap()
    }

    /// Get the length of the body stream, if it has been set.
    ///
    /// This value is set when passing a fixed-size object into as the body. E.g. a string, or a
    /// buffer. Consumers of this API should check this value to decide whether to use `Chunked`
    /// encoding, or set the response length.
    pub fn len(&self) -> Option<usize> {
        self.body.len()
    }

    /// Set the length of the body stream.
    pub fn set_len(&mut self, len: usize) {
        self.body.set_len(len);
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

    /// An iterator visiting all header names in arbitrary order.
    pub fn names<'a>(&'a self) -> Names<'a> {
        self.headers.names()
    }

    /// An iterator visiting all header values in arbitrary order.
    pub fn values<'a>(&'a self) -> Values<'a> {
        self.headers.values()
    }
}

impl Read for Response {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.body).poll_read(cx, buf)
    }
}

impl BufRead for Response {
    #[allow(missing_doc_code_examples)]
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&'_ [u8]>> {
        let this = self.project();
        this.body.poll_fill_buf(cx)
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        Pin::new(&mut self.body).consume(amt)
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

impl Into<Body> for Response {
    fn into(self) -> Body {
        self.body
    }
}

impl From<String> for Response {
    fn from(s: String) -> Self {
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(s);
        res
    }
}

impl<'a> From<&'a str> for Response {
    fn from(s: &'a str) -> Self {
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(s);
        res
    }
}

impl From<Vec<u8>> for Response {
    fn from(b: Vec<u8>) -> Self {
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(b);
        res
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
