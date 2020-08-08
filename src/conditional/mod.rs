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
mod if_modified_since;
mod if_unmodified_since;
mod last_modified;

pub use etag::ETag;
pub use if_modified_since::IfModifiedSince;
pub use if_unmodified_since::IfUnmodifiedSince;
pub use last_modified::LastModified;
