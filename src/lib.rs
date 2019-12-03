//! HTTP Types.
//!
//! ## Example
//!
//! ```rust
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
    pub use url::{
        EncodingOverride, Host, OpaqueOrigin, Origin, ParseError, ParseOptions, PathSegmentsMut,
        Position, SyntaxViolation, Url, UrlQuery,
    };
}

pub mod headers;
pub mod mime;

/// Security headers.
pub mod secure {
    /// An HTTP security policy.
    #[derive(Debug)]
    pub struct Policy {}
}

mod method;
mod request;
mod response;
mod status_code;
mod version;

pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use status_code::StatusCode;
pub use version::HttpVersion;

#[doc(inline)]
pub use headers::Headers;

#[doc(inline)]
pub use crate::url::Url;
