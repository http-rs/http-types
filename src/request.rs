use async_std::io::{self, BufRead, Read};

use std::pin::Pin;
use std::task::{Context, Poll};

use crate::headers::{self, HeaderName, HeaderValue, Headers, Names, ToHeaderValues, Values};
use crate::mime::Mime;
use crate::{Body, Method, Url, Version};

pin_project_lite::pin_project! {
    /// An HTTP request.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_types::{Url, Method, Request};
    ///
    /// let mut req = Request::new(Method::Get, Url::parse("https://example.com").unwrap());
    /// req.set_body("hello world");
    /// ```
    #[derive(Debug)]
    pub struct Request {
        method: Method,
        url: Url,
        headers: Headers,
        version: Option<Version>,
        #[pin]
        body: Body,
    }
}

impl Request {
    /// Create a new request.
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: Headers::new(),
            version: None,
            body: Body::empty(),
        }
    }

    /// Get the HTTP method
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Get a reference to the url.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), http_types::url::ParseError> {
    /// #
    /// use http_types::{Url, Method, Request, Response, StatusCode};
    /// let mut req = Request::new(Method::Get, Url::parse("https://example.com")?);
    /// assert_eq!(req.url().scheme(), "https");
    /// #
    /// # Ok(()) }
    /// ```
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get a mutable reference to the url.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), http_types::url::ParseError> {
    /// #
    /// use http_types::{Url, Method, Request, Response, StatusCode};
    /// let mut req = Request::new(Method::Get, Url::parse("https://example.com")?);
    /// req.url_mut().set_scheme("http");
    /// assert_eq!(req.url().scheme(), "http");
    /// #
    /// # Ok(()) }
    /// ```
    pub fn url_mut(&mut self) -> &mut Url {
        &mut self.url
    }

    /// Get the body.
    pub fn body(&self) -> &Body {
        &self.body
    }

    /// Consume self and get ownership of the body.
    pub fn into_body(self) -> Body {
        self.body
    }

    /// Set the request body.
    pub fn set_body(&mut self, body: impl Into<Body>) {
        self.body = body.into();
        let mime = self.body.take_mime();
        self.set_content_type(mime);
    }

    /// Get an HTTP header.
    pub fn header(&self, name: &HeaderName) -> Option<&Vec<HeaderValue>> {
        self.headers.get(name)
    }

    /// Get a mutable reference to a header.
    pub fn header_mut(&mut self, name: &HeaderName) -> Option<&mut Vec<HeaderValue>> {
        self.headers.get_mut(name)
    }

    /// Remove a header.
    pub fn remove_header(&mut self, name: &HeaderName) -> Option<Vec<HeaderValue>> {
        self.headers.remove(name)
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

    /// Set the response MIME.
    // TODO: return a parsed MIME
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

    /// Get the HTTP version, if one has been set.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_types::{Url, Method, Request, Version};
    ///
    /// # fn main() -> Result<(), http_types::url::ParseError> {
    /// #
    /// let mut req = Request::new(Method::Get, Url::parse("https://example.com")?);
    /// assert_eq!(req.version(), None);
    ///
    /// req.set_version(Some(Version::Http2_0));
    /// assert_eq!(req.version(), Some(Version::Http2_0));
    /// #
    /// # Ok(()) }
    /// ```
    pub fn version(&self) -> Option<Version> {
        self.version
    }

    /// Set the HTTP version.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_types::{Url, Method, Request, Version};
    ///
    /// # fn main() -> Result<(), http_types::url::ParseError> {
    /// #
    /// let mut req = Request::new(Method::Get, Url::parse("https://example.com")?);
    /// req.set_version(Some(Version::Http2_0));
    /// #
    /// # Ok(()) }
    /// ```
    pub fn set_version(&mut self, version: Option<Version>) {
        self.version = version;
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

impl Read for Request {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.body).poll_read(cx, buf)
    }
}

impl BufRead for Request {
    #[allow(missing_doc_code_examples)]
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&'_ [u8]>> {
        let this = self.project();
        this.body.poll_fill_buf(cx)
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        Pin::new(&mut self.body).consume(amt)
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

impl Into<Body> for Request {
    fn into(self) -> Body {
        self.body
    }
}

impl IntoIterator for Request {
    type Item = (HeaderName, Vec<HeaderValue>);
    type IntoIter = headers::IntoIter;

    /// Returns a iterator of references over the remaining items.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.into_iter()
    }
}

impl<'a> IntoIterator for &'a Request {
    type Item = (&'a HeaderName, &'a Vec<HeaderValue>);
    type IntoIter = headers::Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.iter()
    }
}

impl<'a> IntoIterator for &'a mut Request {
    type Item = (&'a HeaderName, &'a mut Vec<HeaderValue>);
    type IntoIter = headers::IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.headers.iter_mut()
    }
}
