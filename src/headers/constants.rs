use super::HeaderName;

/// The `Content-Length` Header
pub const CONTENT_LENGTH: HeaderName = HeaderName::from_lowercase_str("content-length");

/// The `Content-Type` Header
pub const CONTENT_TYPE: HeaderName = HeaderName::from_lowercase_str("content-type");

/// The `Cookie` Header
pub const COOKIE: HeaderName = HeaderName::from_lowercase_str("cookie");

/// The `Set-Cookie` Header
pub const SET_COOKIE: HeaderName = HeaderName::from_lowercase_str("set-cookie");

/// The `Transfer-Encoding` Header
pub const TRANSFER_ENCODING: HeaderName = HeaderName::from_lowercase_str("transfer-encoding");

/// The `Date` Header
pub const DATE: HeaderName = HeaderName::from_lowercase_str("date");

/// The `Origin` Header
pub const ORIGIN: HeaderName = HeaderName::from_lowercase_str("origin");

/// The `access-control-max-age` Header
pub const ACCESS_CONTROL_MAX_AGE: HeaderName =
    HeaderName::from_lowercase_str("access-control-max-age");
/// The `access-control-allow-origin` Header
pub const ACCESS_CONTROL_ALLOW_ORIGIN: HeaderName =
    HeaderName::from_lowercase_str("access-control-allow-origin");
/// The `access-control-allow-headers` Header
pub const ACCESS_CONTROL_ALLOW_HEADERS: HeaderName =
    HeaderName::from_lowercase_str("access-control-allow-headers");
/// The `access-control-allow-methods` Header
pub const ACCESS_CONTROL_ALLOW_METHODS: HeaderName =
    HeaderName::from_lowercase_str("access-control-allow-methods");
/// The `access-control-expose-headers` Header
pub const ACCESS_CONTROL_EXPOSE_HEADERS: HeaderName =
    HeaderName::from_lowercase_str("access-control-expose-headers");
/// The `access-control-request-method` Header
pub const ACCESS_CONTROL_REQUEST_METHOD: HeaderName =
    HeaderName::from_lowercase_str("access-control-request-method");
/// The `access-control-request-headers` Header
pub const ACCESS_CONTROL_REQUEST_HEADERS: HeaderName =
    HeaderName::from_lowercase_str("access-control-request-headers");
/// The `access-control-allow-credentials` Header
pub const ACCESS_CONTROL_ALLOW_CREDENTIALS: HeaderName =
    HeaderName::from_lowercase_str("access-control-allow-credentials");
