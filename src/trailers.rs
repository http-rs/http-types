//! HTTP trailing headers.
//!
//! Trailing headers are headers that are send *after* the body payload has
//! been sent. This is for example useful for sending integrity checks of
//! streamed payloads that are computed on the fly.
//!
//! The way trailing headers are sent over the wire varies per protocol. But in
//! `http-types` we provide a `Trailers` struct that's used to contain the headers.
//!
//! To send trailing headers, see `Request::{`[`send_trailers, `][req_send]
//! [`recv_trailers`][req_recv]`}` and
//! `Response::{`[`send_trailers, `][res_send][`recv_trailers`][res_recv]`}`.
//!
//! [req_send]: ../struct.Request.html#method.send_trailers
//! [req_recv]: ../struct.Request.html#method.recv_trailers
//! [res_send]: ../struct.Response.html#method.send_trailers
//! [res_recv]: ../struct.Response.html#method.recv_trailers
//!
//! ## Example
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # async_std::task::block_on(async {
//! #
//! use http_types::{Url, Method, Request, Trailers};
//! use http_types::headers::{HeaderName, HeaderValue};
//! use async_std::task;
//! use std::str::FromStr;
//!
//! let mut req = Request::new(Method::Get, Url::parse("https://example.com").unwrap());
//!
//! let sender = req.send_trailers();
//! let mut trailers = Trailers::new();
//! trailers.insert("Content-Type", "text/plain")?;
//!
//! task::spawn(async move {
//!     let _trailers = req.recv_trailers().await;
//! });
//!
//! sender.send(Ok(trailers)).await;
//! #
//! # Ok(()) })}
//! ```
//!
//! ## See Also
//! - [MDN HTTP Headers: Trailer](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Trailer)
//! - [HTTP/2 spec: HTTP Sequence](https://http2.github.io/http2-spec/#HttpSequence)

use crate::headers::{
    HeaderName, HeaderValue, Headers, Iter, IterMut, Names, ToHeaderValues, Values,
};
use async_std::sync::Sender;

use std::convert::TryInto;
use std::ops::{Deref, DerefMut};

/// A collection of trailing HTTP headers.
#[derive(Debug, PartialEq)]
pub struct Trailers {
    headers: Headers,
}

impl Trailers {
    /// Create a new instance of `Trailers`.
    pub fn new() -> Self {
        Self {
            headers: Headers::new(),
        }
    }

    /// Insert a header into the headers.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #
    /// use http_types::Trailers;
    ///
    /// let mut trailers = Trailers::new();
    /// trailers.insert("Content-Type", "text/plain")?;
    /// #
    /// # Ok(()) }
    /// ```
    pub fn insert(
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
    /// use http_types::Trailers;
    ///
    /// let mut trailers = Trailers::new();
    /// trailers.append("Content-Type", "text/plain")?;
    /// #
    /// # Ok(()) }
    /// ```
    pub fn append(
        &mut self,
        name: impl TryInto<HeaderName>,
        values: impl ToHeaderValues,
    ) -> crate::Result<()> {
        self.headers.append(name, values)
    }

    /// Get a reference to a header.
    pub fn get(&self, name: &HeaderName) -> Option<&Vec<HeaderValue>> {
        self.headers.get(name)
    }

    /// Get a mutable reference to a header.
    pub fn get_mut(&mut self, name: &HeaderName) -> Option<&mut Vec<HeaderValue>> {
        self.headers.get_mut(name)
    }

    /// Remove a header.
    pub fn remove(&mut self, name: &HeaderName) -> Option<Vec<HeaderValue>> {
        self.headers.remove(name)
    }

    /// An iterator visiting all header pairs in arbitrary order.
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        self.headers.iter()
    }

    /// An iterator visiting all header pairs in arbitrary order, with mutable references to the
    /// values.
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a> {
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

impl Clone for Trailers {
    fn clone(&self) -> Self {
        Self {
            headers: Headers {
                headers: self.headers.headers.clone(),
            },
        }
    }
}

impl Deref for Trailers {
    type Target = Headers;

    fn deref(&self) -> &Self::Target {
        &self.headers
    }
}

impl DerefMut for Trailers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.headers
    }
}

/// The sending half of a channel to send trailers.
///
/// Unlike `async_std::sync::channel` the `send` method on this type can only be
/// called once, and cannot be cloned. That's because only a single instance of
/// `Trailers` should be created.
#[derive(Debug)]
pub struct TrailersSender {
    sender: Sender<crate::Result<Trailers>>,
}

impl TrailersSender {
    /// Create a new instance of `TrailersSender`.
    pub fn new(sender: Sender<crate::Result<Trailers>>) -> Self {
        Self { sender }
    }

    /// Send a `Trailer`.
    ///
    /// The channel will be consumed after having sent trailers.
    pub async fn send(self, trailers: crate::Result<Trailers>) {
        self.sender.send(trailers).await
    }
}
