use super::HeaderName;

/// The `Content-Length` Header
pub const CONTENT_LENGTH: HeaderName = HeaderName::from_lowercase_str("content-length");

/// The `Content-Type` Header
pub const CONTENT_TYPE: HeaderName = HeaderName::from_lowercase_str("content-type");

/// The `Cookie` Header
pub const COOKIE: HeaderName = HeaderName::from_lowercase_str("cookie");

/// The `Set-Cookie` Header
pub const SET_COOKIE: HeaderName = HeaderName::from_lowercase_str("set-cookie");
