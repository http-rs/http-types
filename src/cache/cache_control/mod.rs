//! HTTP `Cache-Control` headers.
//!
//! # Specifications
//!
//! - [RFC 5861: HTTP Cache-Control Extensions for Stale Content](https://tools.ietf.org/html/rfc5861)
//! - [RFC 7234:  Hypertext Transfer Protocol (HTTP/1.1): Caching](https://tools.ietf.org/html/rfc7234)
//! - [RFC 8246: HTTP Immutable Responses](https://tools.ietf.org/html/rfc8246)

mod cache_control;
mod cache_directive;

pub use cache_control::CacheControl;
pub use cache_directive::CacheDirective;
