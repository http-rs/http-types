//! HTTP error types

use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};
use std::mem;

use crate::headers::{
    HeaderName, HeaderValue, HeaderValues, Headers, ToHeaderValues, CONTENT_TYPE,
};
use crate::mime::Mime;
use crate::{Body, StatusCode};

/// A specialized `Result` type for HTTP operations.
///
/// This type is broadly used across `http_types` for any operation which may
/// produce an error.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for HTTP operations.
pub struct Error {
    error: anyhow::Error,
    status: StatusCode,
    headers: Option<Headers>,
    body: Option<Body>,
}

impl Error {
    /// Create a new error object from any error type.
    ///
    /// The error type must be threadsafe and 'static, so that the Error will be
    /// as well. If the error type does not provide a backtrace, a backtrace will
    /// be created here to ensure that a backtrace exists.
    pub fn new<E>(status: impl Into<StatusCode>, error: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self {
            status: status.into(),
            error: anyhow::Error::new(error),
            headers: None,
            body: None,
        }
    }

    /// Create a new error object from static string.
    pub fn from_str<M>(status: impl Into<StatusCode>, msg: M) -> Self
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        Self {
            status: status.into(),
            error: anyhow::Error::msg(msg),
            headers: None,
            body: None,
        }
    }

    /// Create a new error from a message.
    pub(crate) fn new_adhoc<M>(message: M) -> Error
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        Self::from_str(StatusCode::InternalServerError, message)
    }

    /// Get the status code associated with this error.
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Set the status code associated with this error.
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }

    /// Get a mutable reference to a header.
    pub fn header_mut(&mut self, name: impl Into<HeaderName>) -> Option<&mut HeaderValues> {
        self.headers.as_mut()?.get_mut(name.into())
    }

    /// Get an HTTP header.
    pub fn header(&self, name: impl Into<HeaderName>) -> Option<&HeaderValues> {
        self.headers.as_ref()?.get(name.into())
    }

    /// Remove a header.
    pub fn remove_header(&mut self, name: impl Into<HeaderName>) -> Option<HeaderValues> {
        self.headers.as_mut()?.remove(name.into())
    }

    /// Set an HTTP header.
    pub fn insert_header(
        &mut self,
        name: impl Into<HeaderName>,
        values: impl ToHeaderValues,
    ) -> Option<HeaderValues> {
        if self.headers.is_none() {
            self.headers = Some(Headers::new());
        }
        self.headers.as_mut()?.insert(name, values)
    }

    /// Append a header to the headers.
    pub fn append_header(&mut self, name: impl Into<HeaderName>, values: impl ToHeaderValues) {
        if self.headers.is_none() {
            self.headers = Some(Headers::new());
        }
        if let Some(headers) = &mut self.headers {
            headers.append(name, values);
        }
    }

    /// Take the headers.
    pub fn take_headers(&mut self) -> Option<Headers> {
        self.headers.take()
    }

    /// Set the body reader.
    pub fn set_body(&mut self, body: impl Into<Body>) {
        self.body = Some(body.into());
    }

    /// Replace the response body with a new body, returning the old body.
    pub fn replace_body(&mut self, body: Option<impl Into<Body>>) -> Option<Body> {
        let body = match body {
            Some(b) => self.body.replace(b.into()),
            None => self.body.take(),
        };
        self.copy_content_type_from_body();
        body
    }

    /// Swaps the value of the body with another body, without deinitializing
    /// either one.
    pub fn swap_body(&mut self, body: &mut Option<Body>) {
        mem::swap(&mut self.body, body);
        self.copy_content_type_from_body();
    }

    /// Take the response body, replacing it with an empty body.
    pub fn take_body(&mut self) -> Option<Body> {
        self.body.take()
    }

    /// Set the response MIME.
    pub fn set_content_type(&mut self, mime: Mime) -> Option<HeaderValues> {
        let value: HeaderValue = mime.into();

        // A Mime instance is guaranteed to be valid header name.
        self.insert_header(CONTENT_TYPE, value)
    }

    /// Copy MIME data from the body.
    fn copy_content_type_from_body(&mut self) {
        if self.header(CONTENT_TYPE).is_none() && self.body.is_some() {
            self.set_content_type(self.body.as_ref().unwrap().mime().clone());
        }
    }

    /// Get the current content type
    pub fn content_type(&self) -> Option<Mime> {
        self.header(CONTENT_TYPE)?.last().as_str().parse().ok()
    }

    /// Get the length of the body stream, if it has been set.
    ///
    /// This value is set when passing a fixed-size object into as the body.
    /// E.g. a string, or a buffer. Consumers of this API should check this
    /// value to decide whether to use `Chunked` encoding, or set the
    /// response length.
    pub fn len(&self) -> Option<usize> {
        self.body.as_ref()?.len()
    }

    /// Returns `true` if the set length of the body stream is zero, `false`
    /// otherwise.
    pub fn is_empty(&self) -> Option<bool> {
        self.body.as_ref()?.is_empty()
    }

    /// Get the backtrace for this Error.
    ///
    /// Backtraces are only available on the nightly channel. Tracking issue:
    /// [rust-lang/rust#53487][tracking].
    ///
    /// In order for the backtrace to be meaningful, the environment variable
    /// `RUST_LIB_BACKTRACE=1` must be defined. Backtraces are somewhat
    /// expensive to capture in Rust, so we don't necessarily want to be
    /// capturing them all over the place all the time.
    ///
    /// [tracking]: https://github.com/rust-lang/rust/issues/53487
    #[cfg(backtrace)]
    pub fn backtrace(&self) -> &std::backtrace::Backtrace {
        self.error.downcast_ref::<E>()
    }

    /// Attempt to downcast the error object to a concrete type.
    pub fn downcast<E>(self) -> std::result::Result<E, Self>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        if self.error.downcast_ref::<E>().is_some() {
            Ok(self.error.downcast().unwrap())
        } else {
            Err(self)
        }
    }

    /// Downcast this error object by reference.
    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        self.error.downcast_ref::<E>()
    }

    /// Downcast this error object by mutable reference.
    pub fn downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        self.error.downcast_mut::<E>()
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.error)
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.error)
    }
}

impl<E> From<E> for Error
where
    E: StdError + Send + Sync + 'static,
{
    fn from(error: E) -> Self {
        Self {
            error: anyhow::Error::new(error),
            status: StatusCode::InternalServerError,
            headers: None,
            body: None,
        }
    }
}

impl AsRef<dyn StdError + Send + Sync> for Error {
    fn as_ref(&self) -> &(dyn StdError + Send + Sync + 'static) {
        self.error.as_ref()
    }
}

impl AsRef<StatusCode> for Error {
    fn as_ref(&self) -> &StatusCode {
        &self.status
    }
}

impl AsMut<StatusCode> for Error {
    fn as_mut(&mut self) -> &mut StatusCode {
        &mut self.status
    }
}

impl AsRef<dyn StdError> for Error {
    fn as_ref(&self) -> &(dyn StdError + 'static) {
        self.error.as_ref()
    }
}

impl From<Error> for Box<dyn StdError + Send + Sync + 'static> {
    fn from(error: Error) -> Self {
        error.error.into()
    }
}

impl From<Error> for Box<dyn StdError + 'static> {
    fn from(error: Error) -> Self {
        Box::<dyn StdError + Send + Sync>::from(error.error)
    }
}
