//! HTTP Types.
//!
//! ## Example
//!
//! ```rust
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
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

/// IANA Media Types.
pub mod mime {}

/// Security headers.
pub mod secure {
    /// An HTTP security policy.
    #[derive(Debug)]
    pub struct Policy {}
}

mod request;
mod response;
mod method;
mod status_code;

pub use request::Request;
pub use response::Response;
pub use method::Method;
pub use status_code::StatusCode;

#[doc(inline)]
pub use headers::Headers;

#[doc(inline)]
pub use crate::url::Url;
