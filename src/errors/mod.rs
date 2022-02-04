//! HTTP error types
//!
//! This includes two error types for different purposes:
//! One, to either be used as a Response and consumed by a server's middleware, or produced by
//! a client with middleware capabilities; with the ability to dynamically encapsulate
//! any error in handlers (or middleware).
//! Another, to be made by common http operation errors.

mod error_kind;
mod request_error;
mod response_error;

pub use error_kind::*;
pub use request_error::RequestError;
pub use response_error::ResponseError;

/// Result type for errors from http-types.
pub type Result<T> = std::result::Result<T, Error>;

/// Result type for errors from a client making requests using http-types.
pub type RequestResult<T> = std::result::Result<T, RequestError>;

/// Result type for errors provided to a response handler using http-types.
pub type ResponseResult<T> = std::result::Result<T, ResponseError>;
