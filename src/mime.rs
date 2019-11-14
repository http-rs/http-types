//! IANA Media Types.
//!
//! [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types).

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

/// Content-Type for byte streams.
pub const BYTE_STREAM: &'static str = "application/octet-stream";

/// Content-Type for form.
pub const FORM: &'static str = "application/x-www-urlencoded";

/// Content-Type for a multipart form.
pub const MULTIPART_FORM: &'static str = "multipart/form-data";
