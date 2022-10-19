//! HTTP range requests.
//!
//! This submodule includes headers and types to handle HTTP range requests.
//! This allows to address use cases such resuming an interrupted download
//! or downloading a subpart of a large document like a video.
//!
//! The implementation so far is limited to bytes ranges. The specification
//! allows for other types but does not specify any. Range requests using
//! a custom type will have to be processed *manually*, parsing the various
//! headers `Range`, `If-Range`, `Content-Range` ... with the custom type
//! specification.
//!
//! # Further reading
//!
//! - [MDN: HTTP Range Requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests)
//! - [IETF: HTTP Range Requests](https://tools.ietf.org/html/rfc7233)

mod accept_ranges;
mod bytes;
mod content_range;
#[allow(clippy::module_inception)]
mod range;

pub use accept_ranges::AcceptRanges;
pub use bytes::{BytesContentRange, BytesRange, BytesRangeSet};
pub use content_range::ContentRange;
pub use range::Range;
