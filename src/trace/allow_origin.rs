use crate::headers::{HeaderName, HeaderValue, Headers, ToHeaderValues, TIMING_ALLOW_ORIGIN};
use crate::Url;
use std::option;

/// `Timing-Allow-Origin` header.
///
/// # Specifications
///
/// - [W3C Timing-Allow-Origin header](https://w3c.github.io/resource-timing/#sec-timing-allow-origin)
/// - [WhatWG Fetch Origin header](https://fetch.spec.whatwg.org/#origin-header)
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllowOrigin {
    origins: Vec<Origin>,
}

impl AllowOrigin {
    /// Create a new instance of `AllowOrigin`.
    pub fn new() -> Self {
        Self { origins: vec![] }
    }

    /// Create an instance of `AllowOrigin` from a `Headers` instance.
    ///
    /// # Implementation note
    ///
    /// If a `"null"` value is found
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let allow_origin = match headers.as_ref().get(TIMING_ALLOW_ORIGIN) {
            Some(header) => header,
            None => return Ok(None),
        };

        allow_origin.as_str().split(",");
        todo!();
    }

    /// Append an origin to the list of origins.
    pub fn push(&mut self, origin: impl Into<Origin>) {
        self.origins.push(origin.into());
    }

    /// Insert a `HeaderName` + `HeaderValue` pair into a `Headers` instance.
    pub fn apply(&self, headers: impl AsMut<Headers>) {
        todo!();
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        todo!();
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        todo!();
    }
}

// Conversion from `AllowOrigin` -> `HeaderValue`.
impl ToHeaderValues for AllowOrigin {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        todo!()
    }
}

/// An origin passed into `AllowOrigin`.
///
/// Values can either be `Url` or `Wildcard`. `"null"` values are skipped during parsing.
//
// NOTE: this origin is different than the origin in the fetch spec. It needs to
// be its own type.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Origin {
    /// An origin URL.
    Url(Url),
    /// Allow all origins.
    Wildcard,
}

impl From<Url> for Origin {
    fn from(url: Url) -> Self {
        Origin::Url(url)
    }
}

// Conversion from `AllowOrigin` -> `HeaderValue`.
impl ToHeaderValues for Origin {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        let res = unsafe {
            match self {
                Self::Url(url) => {
                    HeaderValue::from_bytes_unchecked(format!("{}", url).into_bytes())
                }
                Self::Wildcard => HeaderValue::from_bytes_unchecked(String::from("*").into_bytes()),
            }
        };
        Ok(Some(res).into_iter())
    }
}
