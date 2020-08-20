//! HTTP conditional headers.
//!
//! Web page performance can be significantly improved by caching resources.
//! This submodule includes headers and types to communicate how and when to
//! cache resources.
//!
//! # Further Reading
//!
//! - [MDN: HTTP Conditional Requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Conditional_requests)

mod etag;

pub use etag::ETag;
