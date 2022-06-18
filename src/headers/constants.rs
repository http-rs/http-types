use super::FieldName;

/// The `Content-Encoding` Header
pub const CONTENT_ENCODING: FieldName = FieldName::from_lowercase_str("content-encoding");
/// The `Content-Language` Header
pub const CONTENT_LANGUAGE: FieldName = FieldName::from_lowercase_str("content-language");
/// The `Content-Length` Header
pub const CONTENT_LENGTH: FieldName = FieldName::from_lowercase_str("content-length");
/// The `Content-Location` Header
pub const CONTENT_LOCATION: FieldName = FieldName::from_lowercase_str("content-location");
/// The `Content-MD5` Header
pub const CONTENT_MD5: FieldName = FieldName::from_lowercase_str("content-md5");
/// The `Content-Range` Header
pub const CONTENT_RANGE: FieldName = FieldName::from_lowercase_str("content-range");
/// The `Content-Type` Header
pub const CONTENT_TYPE: FieldName = FieldName::from_lowercase_str("content-type");

/// The `Cookie` Header
pub const COOKIE: FieldName = FieldName::from_lowercase_str("cookie");

/// The `Set-Cookie` Header
pub const SET_COOKIE: FieldName = FieldName::from_lowercase_str("set-cookie");

/// The `Transfer-Encoding` Header
pub const TRANSFER_ENCODING: FieldName = FieldName::from_lowercase_str("transfer-encoding");

/// The `Date` Header
pub const DATE: FieldName = FieldName::from_lowercase_str("date");

/// The `Host` Header
pub const HOST: FieldName = FieldName::from_lowercase_str("host");

/// The `Origin` Header
pub const ORIGIN: FieldName = FieldName::from_lowercase_str("origin");

/// The `access-control-max-age` Header
pub const ACCESS_CONTROL_MAX_AGE: FieldName =
    FieldName::from_lowercase_str("access-control-max-age");
/// The `access-control-allow-origin` Header
pub const ACCESS_CONTROL_ALLOW_ORIGIN: FieldName =
    FieldName::from_lowercase_str("access-control-allow-origin");
/// The `access-control-allow-headers` Header
pub const ACCESS_CONTROL_ALLOW_HEADERS: FieldName =
    FieldName::from_lowercase_str("access-control-allow-headers");
/// The `access-control-allow-methods` Header
pub const ACCESS_CONTROL_ALLOW_METHODS: FieldName =
    FieldName::from_lowercase_str("access-control-allow-methods");
/// The `access-control-expose-headers` Header
pub const ACCESS_CONTROL_EXPOSE_HEADERS: FieldName =
    FieldName::from_lowercase_str("access-control-expose-headers");
/// The `access-control-request-method` Header
pub const ACCESS_CONTROL_REQUEST_METHOD: FieldName =
    FieldName::from_lowercase_str("access-control-request-method");
/// The `access-control-request-headers` Header
pub const ACCESS_CONTROL_REQUEST_HEADERS: FieldName =
    FieldName::from_lowercase_str("access-control-request-headers");
/// The `access-control-allow-credentials` Header
pub const ACCESS_CONTROL_ALLOW_CREDENTIALS: FieldName =
    FieldName::from_lowercase_str("access-control-allow-credentials");

///  The `Accept` Header
pub const ACCEPT: FieldName = FieldName::from_lowercase_str("accept");
///  The `Accept-Charset` Header
pub const ACCEPT_CHARSET: FieldName = FieldName::from_lowercase_str("accept-charset");
///  The `Accept-Encoding` Header
pub const ACCEPT_ENCODING: FieldName = FieldName::from_lowercase_str("accept-encoding");
///  The `Accept-Language` Header
pub const ACCEPT_LANGUAGE: FieldName = FieldName::from_lowercase_str("accept-language");
///  The `Accept-Ranges` Header
pub const ACCEPT_RANGES: FieldName = FieldName::from_lowercase_str("accept-ranges");

///  The `Age` Header
pub const AGE: FieldName = FieldName::from_lowercase_str("age");

///  The `Allow` Header
pub const ALLOW: FieldName = FieldName::from_lowercase_str("allow");

///  The `Authorization` Header
pub const AUTHORIZATION: FieldName = FieldName::from_lowercase_str("authorization");

///  The `Cache-Control` Header
pub const CACHE_CONTROL: FieldName = FieldName::from_lowercase_str("cache-control");

///  The `Clear-Site-Data` Header
pub const CLEAR_SITE_DATA: FieldName = FieldName::from_lowercase_str("clear-site-data");

///  The `Connection` Header
pub const CONNECTION: FieldName = FieldName::from_lowercase_str("connection");

///  The `ETag` Header
pub const ETAG: FieldName = FieldName::from_lowercase_str("etag");

///  The `Expect` Header
pub const EXPECT: FieldName = FieldName::from_lowercase_str("expect");

///  The `Expires` Header
pub const EXPIRES: FieldName = FieldName::from_lowercase_str("expires");

/// The `Forwarded` Header
pub const FORWARDED: FieldName = FieldName::from_lowercase_str("forwarded");

///  The `From` Header
pub const FROM: FieldName = FieldName::from_lowercase_str("from");

///  The `If-Match` Header
pub const IF_MATCH: FieldName = FieldName::from_lowercase_str("if-match");

///  The `If-Modified-Since` Header
pub const IF_MODIFIED_SINCE: FieldName = FieldName::from_lowercase_str("if-modified-since");

///  The `If-None-Match` Header
pub const IF_NONE_MATCH: FieldName = FieldName::from_lowercase_str("if-none-match");

///  The `If-Range` Header
pub const IF_RANGE: FieldName = FieldName::from_lowercase_str("if-range");

///  The `If-Unmodified-Since` Header
pub const IF_UNMODIFIED_SINCE: FieldName = FieldName::from_lowercase_str("if-unmodified-since");

///  The `Last-Modified` Header
pub const LAST_MODIFIED: FieldName = FieldName::from_lowercase_str("last-modified");

///  The `Location` Header
pub const LOCATION: FieldName = FieldName::from_lowercase_str("location");

///  The `Max-Forwards` Header
pub const MAX_FORWARDS: FieldName = FieldName::from_lowercase_str("max-forwards");

///  The `Pragma` Header
pub const PRAGMA: FieldName = FieldName::from_lowercase_str("pragma");

///  The `Proxy-Authenticate` Header
pub const PROXY_AUTHENTICATE: FieldName = FieldName::from_lowercase_str("proxy-authenticate");

///  The `Proxy-Authorization` Header
pub const PROXY_AUTHORIZATION: FieldName = FieldName::from_lowercase_str("proxy-authorization");

/// The `Proxy-Connection` Header
pub const PROXY_CONNECTION: FieldName = FieldName::from_lowercase_str("proxy-connection");

///  The `Referer` Header
pub const REFERER: FieldName = FieldName::from_lowercase_str("referer");

///  The `Retry-After` Header
pub const RETRY_AFTER: FieldName = FieldName::from_lowercase_str("retry-after");

///  The `Server` Header
pub const SERVER: FieldName = FieldName::from_lowercase_str("server");

///  The `Server` Header
pub const SERVER_TIMING: FieldName = FieldName::from_lowercase_str("server-timing");

///  The `SourceMap` Header
pub const SOURCE_MAP: FieldName = FieldName::from_lowercase_str("sourcemap");

///  The `Strict-Transport-Security` Header
pub const STRICT_TRANSPORT_SECURITY: FieldName =
    FieldName::from_lowercase_str("strict-transport-security");

///  The `Te` Header
pub const TE: FieldName = FieldName::from_lowercase_str("te");

///  The `Timing-Allow-Origin` Header
pub const TIMING_ALLOW_ORIGIN: FieldName = FieldName::from_lowercase_str("timing-allow-origin");

///  The `Traceparent` Header
pub const TRACEPARENT: FieldName = FieldName::from_lowercase_str("traceparent");

///  The `Trailer` Header
pub const TRAILER: FieldName = FieldName::from_lowercase_str("trailer");

///  The `Upgrade` Header
pub const UPGRADE: FieldName = FieldName::from_lowercase_str("upgrade");

///  The `User-Agent` Header
pub const USER_AGENT: FieldName = FieldName::from_lowercase_str("user-agent");

///  The `Vary` Header
pub const VARY: FieldName = FieldName::from_lowercase_str("vary");

///  The `Via` Header
pub const VIA: FieldName = FieldName::from_lowercase_str("via");

///  The `Warning` Header
pub const WARNING: FieldName = FieldName::from_lowercase_str("warning");

///  The `WWW-Authenticate` Header
pub const WWW_AUTHENTICATE: FieldName = FieldName::from_lowercase_str("www-authenticate");
