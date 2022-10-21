use miette::Diagnostic;

use crate::auth::AuthenticationScheme;
use crate::StatusCode;

/// Error kind for http-types
#[derive(Debug, Diagnostic, thiserror::Error)]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Error {
    /// This only happens for APIs which support `TryInto` for an argument.
    #[error("An argument failed to convert during TryInto: {}", .0)]
    ArgTryIntoError(Box<dyn std::error::Error + Send + Sync + 'static>),

    #[error("HTTP Method was unrecognized: {}", .0)]
    HttpMethodUnrecognized(String),

    #[error("I/O error: {}", .0)]
    IO(#[from] std::io::Error),

    #[error("Invalid Status Code: {}", .0)]
    StatusCodeInvalid(u16),

    #[error("Query string deserialization error: {}", .0)]
    QueryDeserialize(serde_qs::Error),
    #[error("Query string serialization error: {}", .0)]
    QuerySerialize(serde_qs::Error),

    #[error("Body error: {}", .0)]
    Body(BodyError),

    #[error("Header error: {}", .0)]
    Header(HeaderError),

    #[cfg(feature = "hyperium_http")]
    #[error("URL Parse error: {}", .0)]
    URLParse(#[from] url::ParseError),

    #[cfg(feature = "hyperium_http")]
    #[error("Hyperium HTTP error: {}", .0)]
    HyperiumHttp(#[from] http::Error),
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[non_exhaustive]
pub enum BodyError {
    #[error("Body size too large: {:?} (PayloadTooLarge)", .0)]
    PayloadTooLarge(Option<u64>),

    // Deserialization
    #[error("Failed to Deserialize JSON: {:?}", .0)]
    DeserializeJSON(serde_json::Error),
    #[error("Failed to Deserialize x-form-urlencoded: {:?}", .0)]
    DeserializeForm(#[from] serde_urlencoded::de::Error),
    // #[error("Failed to Deserialize utf8: {:?}", .0)]
    // DeserializeUTF8(#[from] std::str::Utf8Error),

    // Serialization
    #[error("JSON: {:?}", .0)]
    SerializeJSON(serde_json::Error),
    #[error("x-form-urlencoded: {:?}", .0)]
    SerializeForm(#[from] serde_urlencoded::ser::Error),
}

impl From<BodyError> for Error {
    fn from(other: BodyError) -> Self {
        Error::Body(other)
    }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[non_exhaustive]
pub enum HeaderError {
    // Parse
    #[error("Header value specificity was invalid")]
    SpecificityInvalid,
    /// Header name was invalid
    #[error("Header name was invalid: {}", .0)]
    NameInvalid(&'static str),
    /// Header value was invalid
    #[error("Header value was invalid: {}", .0)]
    ValueInvalid(&'static str),

    // Parse specific headers
    #[error("Date header was invalid: {}", .0)]
    DateInvalid(DateError),
    #[error("No suitable Transfer-Encoding found during negotiation")]
    TransferEncodingUnnegotiable,
    #[error("Transfer-Encoding header encoding was invalid: {}", .0)]
    TransferEncodingInvalidEncoding(EncodingError),
    #[error("Trace-Context header was invalid: {}", .0)]
    TraceContextInvalid(&'static str),
    #[error("Server-Timing header was invalid: {}", .0)]
    ServerTimingInvalid(&'static str),
    #[error("Server-Timing header metric was invalid: {}", .0)]
    ServerTimingInvalidMetric(&'static str),
    #[error("Timing-Allow-Origin header was invalid: {:?}", .0)]
    TimingAllowOriginInvalidUrl(url::ParseError),
    #[error("Forwarded header was invalid: {}", .0)]
    ForwardedInvalid(&'static str),
    #[error("Sourcemap header url was invalid: {:?}", .0)]
    SourceMapInvalidUrl(url::ParseError),
    #[error("Sourcemap header base url was invalid: {:?}", .0)]
    SourceMapInvalidBaseUrl(Box<dyn std::fmt::Debug + Send + Sync + 'static>),
    #[error("Referer header url was invalid: {:?}", .0)]
    RefererInvalidUrl(url::ParseError),
    #[error("Referer header base url was invalid: {:?}", .0)]
    RefererInvalidBaseUrl(Box<dyn std::fmt::Debug + Send + Sync + 'static>),
    #[error("Content-Type header MediaType (MIME) was invalid: {}", .0)]
    ContentTypeInvalidMediaType(MediaTypeError),
    #[error("Content-Length header was invalid: length out of bounds (unsized 64 bit integer)")]
    ContentLengthInvalid,
    #[error("Accept header was invalid: {}", .0)]
    AcceptInvalidMediaType(MediaTypeError),
    #[error("No suitable Content-Type header MediaType found during Accept negotiation")]
    AcceptUnnegotiable,
    #[error("Accept-Encoding header was invalid: {}", .0)]
    AcceptEncodingInvalidEncoding(EncodingError),
    #[error(
        "No suitable Content-Encoding header Encoding found during Accept-Encoding negotiation"
    )]
    AcceptEncodingUnnegotiable,
    #[error("ETag header was invalid")]
    ETagInvalid,
    #[error("Age header was invalid: length out of bounds (unsized 64 bit integer)")]
    AgeInvalid,
    #[error("Clear-Site-Data header was invalid: {}", .0)]
    ClearSiteDataInvalid(std::string::ParseError),
    #[error("Cache-Control header was invalid")]
    CacheControlInvalid,
    #[error("Authorization header was invalid: {}", .0)]
    AuthorizationInvalid(AuthError),
    #[error("WWW-Authenticate header was invalid: {}", .0)]
    WWWAuthenticateInvalid(&'static str),
    #[error("Content-Location header url was invalid: {:?}", .0)]
    ContentLocationInvalidUrl(url::ParseError),
    #[error("Content-Location header base url was invalid: {:?}", .0)]
    ContentLocationInvalidBaseUrl(Box<dyn std::fmt::Debug + Send + Sync + 'static>),
    #[error("Expect header was malformed.")]
    ExpectInvalid,
    #[error("Strict-Transport-Security header was invalid: {:?}", .0)]
    StrictTransportSecurityInvalid(&'static str),
}

impl From<HeaderError> for Error {
    fn from(other: HeaderError) -> Self {
        Error::Header(other)
    }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("`{}` Auth had invalid credentials: {}", .0, .1)]
    CredentialsInvalid(AuthenticationScheme, &'static str),
    #[error("`{}` is not a recognized auth scheme.", .0)]
    SchemeUnrecognized(String),
    #[error("Could not find auth scheme")]
    SchemeMissing,
    #[error("Could not find auth credentials")]
    CredentialsMissing,
    #[error("Expected `{}` auth scheme but found `{}`", .0, .1)]
    SchemeUnexpected(AuthenticationScheme, String),
    #[error("Could not find www-auth realm")]
    RealmMissing,
}

impl From<AuthError> for Error {
    fn from(other: AuthError) -> Self {
        Error::Header(other.into())
    }
}

impl From<AuthError> for HeaderError {
    fn from(other: AuthError) -> Self {
        HeaderError::AuthorizationInvalid(other)
    }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[non_exhaustive]
pub enum DateError {
    #[error("HTTP Date-Time not in {} format", .0)]
    FormatInvalid(&'static str),

    #[error("HTTP Date-Time failed all parsings: imf_fixdate, rfc850, asctime")]
    Unparseable,

    #[error("HTTP Date-Time invalid: parts out of logical bounds")]
    OutOfBounds,

    #[error("HTTP Date-Time string was not ASCII")]
    NotASCII,

    // Individual parts
    #[error("HTTP Date-Time invalid seconds")]
    SecondsInvalid,
    #[error("HTTP Date-Time invalid minutes")]
    MinutesInvalid,
    #[error("HTTP Date-Time invalid hours")]
    HourInvalid,
    #[error("HTTP Date-Time invalid day")]
    DayInvalid,
    #[error("HTTP Date-Time invalid month")]
    MonthInvalid,
    #[error("HTTP Date-Time invalid year")]
    YearInvalid,
    #[error("HTTP Date-Time invalid week-day")]
    WeekdayInvalid,
}

impl From<DateError> for Error {
    fn from(other: DateError) -> Self {
        Error::Header(other.into())
    }
}

impl From<DateError> for HeaderError {
    fn from(other: DateError) -> Self {
        HeaderError::DateInvalid(other)
    }
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[non_exhaustive]
pub enum MediaTypeError {
    #[error("MediaType (MIME) parse error: {}", .0)]
    Parse(&'static str),
    #[error("MediaType (MIME) invalid Param name: {}", .0)]
    ParamName(&'static str),
    #[error("MediaType (MIME) invalid proposal: {}", .0)]
    Proposal(&'static str),
    #[error("Media Type (MIME) could not be sniffed / inferred")]
    Sniff,
}

#[derive(Debug, Diagnostic, thiserror::Error)]
#[non_exhaustive]
pub enum EncodingError {
    #[error("Encoding parse error: {}", .0)]
    Parse(&'static str),
    #[error("Encoding invalid proposal: {}", .0)]
    Proposal(&'static str),
}

impl Error {
    /// Maps this error to its associated http status code, if one logically exists.
    ///
    /// `None` is returned in cases where a default should be used.
    /// It is suggested that frameworks using this code map these to 500 by default when there is no other developer intervention.
    /// (This is what Tide does.)
    pub fn associated_status_code(&self) -> Option<StatusCode> {
        use Error::*;
        match self {
            QueryDeserialize(_) => Some(StatusCode::BadRequest), // XXX(Jeremiah): should this also be 422?
            Body(inner) => inner.associated_status_code(),
            Header(inner) => inner.associated_status_code(),
            _ => None,
        }
    }
}

impl BodyError {
    /// Maps this error to its associated http status code, if one logically exists.
    ///
    /// `None` is returned in cases where a default should be used.
    /// It is suggested that frameworks using this code map these to 500 by default when there is no other developer intervention.
    /// (This is what Tide does.)
    pub fn associated_status_code(&self) -> Option<StatusCode> {
        use BodyError::*;
        match self {
            PayloadTooLarge(_) => Some(StatusCode::PayloadTooLarge),
            DeserializeJSON(_) => Some(StatusCode::UnprocessableEntity),
            DeserializeForm(_) => Some(StatusCode::UnprocessableEntity),
            // XXX(Jeremiah): This is currently a std::io::Error but should probably be mapped to this
            // BodyError::DeserializeUTF8(_) => Some(StatusCode::UnprocessableEntity),
            _ => None,
        }
    }
}

impl HeaderError {
    /// Maps this error to its associated http status code, if one logically exists.
    ///
    /// `None` is returned in cases where a default should be used.
    /// It is suggested that frameworks using this code map these to 500 by default when there is no other developer intervention.
    /// (This is what Tide does.)
    pub fn associated_status_code(&self) -> Option<StatusCode> {
        use HeaderError::*;
        use StatusCode::*;
        match self {
            SpecificityInvalid => Some(BadRequest),

            DateInvalid(_) => Some(BadRequest),
            TransferEncodingUnnegotiable => Some(NotAcceptable),
            TransferEncodingInvalidEncoding(_) => Some(BadRequest),
            TraceContextInvalid(_) => Some(BadRequest),
            ServerTimingInvalid(_) => Some(BadRequest),
            TimingAllowOriginInvalidUrl(_) => Some(BadRequest),
            ForwardedInvalid(_) => Some(BadRequest),
            ContentTypeInvalidMediaType(_) => Some(BadRequest),
            ContentLengthInvalid => Some(BadRequest),
            AcceptInvalidMediaType(_) => Some(BadRequest),
            AcceptUnnegotiable => Some(NotAcceptable),
            AcceptEncodingInvalidEncoding(_) => Some(BadRequest),
            AcceptEncodingUnnegotiable => Some(NotAcceptable),
            ETagInvalid => Some(BadRequest),
            AgeInvalid => Some(BadRequest),
            CacheControlInvalid => Some(BadRequest),
            AuthorizationInvalid(_) => Some(BadRequest),
            WWWAuthenticateInvalid(_) => Some(BadRequest),
            ExpectInvalid => Some(BadRequest),
            StrictTransportSecurityInvalid(_) => Some(BadRequest),

            _ => None,
        }
    }
}
