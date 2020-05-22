use async_std::io::{self, BufRead, Read};
use async_std::sync;

use std::convert::TryInto;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::headers::{
    self, HeaderName, HeaderValue, Headers, Names, ToHeaderValues, Values, CONTENT_TYPE,
};
use crate::mime::Mime;
use crate::trailers::{Trailers, TrailersSender};
use crate::{Body, Cookie, StatusCode, TypeMap, Version};

pin_project_lite::pin_project! {
    /// An HTTP response.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), http_types::Error> {
    /// #
    /// use http_types::{Response, StatusCode};
    ///
    /// let mut res = Response::new(StatusCode::Ok);
    /// res.set_body("Hello, Nori!");
    /// #
    /// # Ok(()) }
    /// ```
    #[derive(Debug)]
    pub struct Response {
        status: StatusCode,
        headers: Headers,
        version: Option<Version>,
        sender: Option<sync::Sender<crate::Result<Trailers>>>,
        receiver: sync::Receiver<crate::Result<Trailers>>,
        #[pin]
        body: Body,
        local: TypeMap,
    }
}

impl Response {
    /// Create a new response.
    pub fn new(status: StatusCode) -> Self {
        let (sender, receiver) = sync::channel(1);
        Self {
            status,
            headers: Headers::new(),
            version: None,
            body: Body::empty(),
            sender: Some(sender),
            receiver,
            local: TypeMap::new(),
        }
    }

    /// Get the status
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get a mutable reference to a header.
    pub fn header_mut(&mut self, name: &HeaderName) -> Option<&mut Vec<HeaderValue>> {
        self.headers.get_mut(name)
    }

    /// Get an HTTP header.
    pub fn header(&self, name: &HeaderName) -> Option<&Vec<HeaderValue>> {
        self.headers.get(name)
    }

    /// Remove a header.
    pub fn remove_header(&mut self, name: &HeaderName) -> Option<Vec<HeaderValue>> {
        self.headers.remove(name)
    }

    /// Set an HTTP header.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #
    /// use http_types::{Url, Method, Response, StatusCode};
    ///
    /// let mut req = Response::new(StatusCode::Ok);
    /// req.insert_header("Content-Type", "text/plain")?;
    /// #
    /// # Ok(()) }
    /// ```
    pub fn insert_header(
        &mut self,
        name: impl TryInto<HeaderName>,
        values: impl ToHeaderValues,
    ) -> crate::Result<Option<Vec<HeaderValue>>> {
        self.headers.insert(name, values)
    }

    /// Append a header to the headers.
    ///
    /// Unlike `insert` this function will not override the contents of a header, but insert a
    /// header if there aren't any. Or else append to the existing list of headers.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #
    /// use http_types::{Response, StatusCode};
    ///
    /// let mut res = Response::new(StatusCode::Ok);
    /// res.append_header("Content-Type", "text/plain")?;
    /// #
    /// # Ok(()) }
    /// ```
    pub fn append_header(
        &mut self,
        name: impl TryInto<HeaderName>,
        values: impl ToHeaderValues,
    ) -> crate::Result<()> {
        self.headers.append(name, values)
    }

    /// Set the body reader.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), http_types::Error> {
    /// #
    /// use http_types::{Response, StatusCode};
    ///
    /// let mut res = Response::new(StatusCode::Ok);
    /// res.set_body("Hello, Nori!");
    /// #
    /// # Ok(()) }
    /// ```
    pub fn set_body(&mut self, body: impl Into<Body>) {
        self.replace_body(body);
    }

    /// Replace the response body with a new body, returning the old body.
    ///
    /// # Examples
    ///
    /// ```
    /// # use async_std::io::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # async_std::task::block_on(async {
    /// #
    /// use http_types::{Body, Url, Method, Response, StatusCode};
    ///
    /// let mut req = Response::new(StatusCode::Ok);
    /// req.set_body("Hello, Nori!");
    ///
    /// let mut body: Body = req.replace_body("Hello, Chashu");
    ///
    /// let mut string = String::new();
    /// body.read_to_string(&mut string).await?;
    /// assert_eq!(&string, "Hello, Nori!");
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn replace_body(&mut self, body: impl Into<Body>) -> Body {
        let body = mem::replace(&mut self.body, body.into());
        self.copy_content_type_from_body();
        body
    }

    /// Swaps the value of the body with another body, without deinitializing
    /// either one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use async_std::io::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # async_std::task::block_on(async {
    /// #
    /// use http_types::{Body, Url, Method, Response, StatusCode};
    ///
    /// let mut req = Response::new(StatusCode::Ok);
    /// req.set_body("Hello, Nori!");
    ///
    /// let mut body = "Hello, Chashu!".into();
    /// req.swap_body(&mut body);
    ///
    /// let mut string = String::new();
    /// body.read_to_string(&mut string).await?;
    /// assert_eq!(&string, "Hello, Nori!");
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn swap_body(&mut self, body: &mut Body) {
        mem::swap(&mut self.body, body);
        self.copy_content_type_from_body();
    }

    /// Take the response body, replacing it with an empty body.
    ///
    /// # Examples
    ///
    /// ```
    /// # use async_std::io::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # async_std::task::block_on(async {
    /// #
    /// use http_types::{Body, Url, Method, Response, StatusCode};
    ///
    /// let mut req = Response::new(StatusCode::Ok);
    /// req.set_body("Hello, Nori!");
    /// let mut body: Body = req.take_body();
    ///
    /// let mut string = String::new();
    /// body.read_to_string(&mut string).await?;
    /// assert_eq!(&string, "Hello, Nori!");
    ///
    /// # let mut string = String::new();
    /// # req.read_to_string(&mut string).await?;
    /// # assert_eq!(&string, "");
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn take_body(&mut self) -> Body {
        self.replace_body(Body::empty())
    }

    /// Read the body as a string.
    ///
    /// This consumes the response. If you want to read the body without
    /// consuming the response, consider using the `take_body` method and
    /// then calling `Body::into_string` or using the Response's AsyncRead
    /// implementation to read the body.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io::prelude::*;
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// # async_std::task::block_on(async {
    /// use http_types::{Body, Url, Method, Response, StatusCode};    
    /// use async_std::io::Cursor;
    ///
    /// let mut resp = Response::new(StatusCode::Ok);    
    /// let cursor = Cursor::new("Hello Nori");
    /// let body = Body::from_reader(cursor, None);
    /// resp.set_body(body);
    /// assert_eq!(&resp.body_string().await.unwrap(), "Hello Nori");
    /// # Ok(()) }) }
    /// ```
    pub async fn body_string(self) -> io::Result<String> {
        self.body.into_string().await
    }

    /// Set the response MIME.
    pub fn set_content_type(&mut self, mime: Mime) -> Option<Vec<HeaderValue>> {
        let value: HeaderValue = mime.into();

        // A Mime instance is guaranteed to be valid header name.
        self.insert_header(CONTENT_TYPE, value).unwrap()
    }

    /// Copy MIME data from the body.
    fn copy_content_type_from_body(&mut self) {
        if self.header(&CONTENT_TYPE).is_none() {
            self.set_content_type(self.body.mime().clone());
        }
    }

    /// Get the length of the body stream, if it has been set.
    ///
    /// This value is set when passing a fixed-size object into as the body. E.g. a string, or a
    /// buffer. Consumers of this API should check this value to decide whether to use `Chunked`
    /// encoding, or set the response length.
    pub fn len(&self) -> Option<usize> {
        self.body.len()
    }

    /// Returns `true` if the set length of the body stream is zero, `false` otherwise.
    pub fn is_empty(&self) -> Option<bool> {
        self.body.is_empty()
    }

    /// Get the HTTP version, if one has been set.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), http_types::Error> {
    /// #
    /// use http_types::{Response, StatusCode, Version};
    ///
    /// let mut res = Response::new(StatusCode::Ok);
    /// assert_eq!(res.version(), None);
    ///
    /// res.set_version(Some(Version::Http2_0));
    /// assert_eq!(res.version(), Some(Version::Http2_0));
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
    /// # fn main() -> Result<(), http_types::Error> {
    /// #
    /// use http_types::{Response, StatusCode, Version};
    ///
    /// let mut res = Response::new(StatusCode::Ok);
    /// res.set_version(Some(Version::Http2_0));
    /// #
    /// # Ok(()) }
    /// ```
    pub fn set_version(&mut self, version: Option<Version>) {
        self.version = version;
    }

    /// Set the status.
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }

    /// Get all cookies.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), http_types::Error> {
    /// #
    /// use http_types::{Cookie, Response, StatusCode, Version};
    ///
    /// let mut res = Response::new(StatusCode::Ok);
    /// res.set_cookie(Cookie::new("name", "value"));
    /// assert_eq!(res.cookies().unwrap(), vec![Cookie::new("name", "value")]);
    /// #
    /// # Ok(()) }
    /// ```
    pub fn cookies(&self) -> Result<Vec<Cookie<'_>>, crate::Error> {
        match self.header(&headers::SET_COOKIE) {
            None => Ok(vec![]),
            Some(h) => h.iter().try_fold(vec![], |mut acc, h| {
                let cookie = Cookie::parse(h.as_str())?;
                acc.push(cookie);
                Ok(acc)
            }),
        }
    }

    /// Get a cookie by name.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), http_types::Error> {
    /// #
    /// use http_types::{Cookie, Response, StatusCode, Version};
    ///
    /// let mut res = Response::new(StatusCode::Ok);
    /// res.set_cookie(Cookie::new("name", "value"));
    /// assert_eq!(res.cookie("name").unwrap(), Some(Cookie::new("name", "value")));
    /// #
    /// # Ok(()) }
    /// ```
    pub fn cookie(&self, name: &str) -> Result<Option<Cookie<'_>>, crate::Error> {
        let cookies = self.cookies()?;
        let cookie = cookies.into_iter().find(|c| c.name() == name);
        Ok(cookie)
    }

    /// Set a cookie.
    ///
    /// This will not override any existing cookies, and uses the `Cookies` header.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), http_types::Error> {
    /// #
    /// use http_types::{Cookie, Response, StatusCode, Version};
    ///
    /// let mut res = Response::new(StatusCode::Ok);
    /// res.set_cookie(Cookie::new("name", "value"));
    /// #
    /// # Ok(()) }
    /// ```
    pub fn set_cookie(&mut self, cookie: Cookie<'_>) {
        self.append_header(headers::SET_COOKIE, HeaderValue::from(cookie))
            .unwrap();
    }

    /// Sends trailers to the a receiver.
    pub fn send_trailers(&mut self) -> TrailersSender {
        let sender = self
            .sender
            .take()
            .expect("Trailers sender can only be constructed once");
        TrailersSender::new(sender)
    }

    /// Receive trailers from a sender.
    pub async fn recv_trailers(&self) -> Option<crate::Result<Trailers>> {
        self.receiver.recv().await.ok()
    }

    /// An iterator visiting all header pairs in arbitrary order.
    pub fn iter(&self) -> headers::Iter<'_> {
        self.headers.iter()
    }

    /// An iterator visiting all header pairs in arbitrary order, with mutable references to the
    /// values.
    pub fn iter_mut(&mut self) -> headers::IterMut<'_> {
        self.headers.iter_mut()
    }

    /// An iterator visiting all header names in arbitrary order.
    pub fn header_names(&self) -> Names<'_> {
        self.headers.names()
    }

    /// An iterator visiting all header values in arbitrary order.
    pub fn header_values(&self) -> Values<'_> {
        self.headers.values()
    }

    /// Returns a reference to the existing local.
    pub fn local(&self) -> &TypeMap {
        &self.local
    }

    /// Returns a mutuable reference to the existing local state.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), http_types::Error> {
    /// #
    /// use http_types::{StatusCode, Response, Version};
    ///
    /// let mut res = Response::new(StatusCode::Ok);
    /// res.local_mut().insert("hello from the extension");
    /// assert_eq!(res.local().get(), Some(&"hello from the extension"));
    /// #
    /// # Ok(()) }
    /// ```
    pub fn local_mut(&mut self) -> &mut TypeMap {
        &mut self.local
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

impl From<Response> for Body {
    fn from(res: Response) -> Body {
        res.body
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
