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
///
/// [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types).
pub mod mime {
    /// Content-Type for JavaScript.
    pub const JAVASCRIPT: &'static str = "application/javascript; charset=utf-8";

    /// Content-Type for JSON.
    pub const JSON: &'static str = "application/json";

    /// Content-Type for CSS.
    pub const CSS: &'static str = "text/css; charset=utf-8";

    /// Content-Type for HTML.
    pub const HTML: &'static str = "text/html; charset=utf-8";

    /// Content-Type for Server Sent Events
    pub const SSE: &'static str = "text/event-stream;";

    /// Content-Type for plain text.
    pub const PLAIN: &'static str = "text/plain; charset=utf-8";

    /// Content-Type for Form.
    pub const FORM: &'static str = "application/x-www-urlencoded";

    /// Content-Type for a multipart form.
    pub const MULTIPART_FORM: &'static str = "multipart/form-data";
}

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

pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use status_code::StatusCode;

#[doc(inline)]
pub use headers::Headers;

#[doc(inline)]
pub use crate::url::Url;
