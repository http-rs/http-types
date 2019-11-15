//! HTTP Types.
//!
//! HTTP types is a library of common HTTP types that serves as a foundation for dedicated HTTP
//! clients, servers, and everything in between. It's built equal parts to be performant, complete,
//! and accessible.
//!
//! ## Example
//!
//! Create a new request:
//!
//! ```
//! use http_types::{Request, Method, mime, Url};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
//! #
//! let url = Url::parse("https://httpbin.org/get")?;
//! let req = Request::new(Method::Get, url)
//!     .set_header("x-cat-name", "chashu")?
//!     .body_string("meow".to_string());
//! #
//! # Ok(())}
//! ```
//!
//! Create a new response:
//!
//! ```
//! use http_types::{Response, Method, mime, Url};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
//! #
//! let url = Url::parse("https://httpbin.org/get")?;
//! let req = Response::new(200)
//!     .set_header("x-cat-name", "nori")?
//!     .body_string("meow".to_string());
//! #
//! # Ok(())}
//! ```

#![forbid(rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

/// HTTP cookies.
pub mod cookies {
    pub use cookie::*;
}

/// URL records.
pub mod url {
    pub use url::*;
}

pub mod headers;
pub mod mime;
pub mod secure;

mod method;
mod request;
mod response;
mod status_code;

pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use status_code::StatusCode;

#[doc(inline)]
pub use headers::Headers;

#[doc(inline)]
pub use crate::url::Url;
