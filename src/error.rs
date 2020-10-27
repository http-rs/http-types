//! HTTP error types

use std::convert::TryInto;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};

use crate::StatusCode;

#[cfg(all(not(backtrace), feature = "error_eyre"))]
use stable_eyre::BacktraceExt;

#[cfg(feature = "error_anyhow")]
use anyhow::Error as BaseError;
#[cfg(feature = "error_eyre")]
use eyre::Report as BaseError;

/// A specialized `Result` type for HTTP operations.
///
/// This type is broadly used across `http_types` for any operation which may
/// produce an error.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for HTTP operations.
pub struct Error {
    error: BaseError,
    status: crate::StatusCode,
    type_name: Option<&'static str>,
}

impl Error {
    /// Create a new error object from any error type.
    ///
    /// The error type must be threadsafe and 'static, so that the Error will be
    /// as well. If the error type does not provide a backtrace, a backtrace will
    /// be created here to ensure that a backtrace exists.
    pub fn new<S, E>(status: S, error: E) -> Self
    where
        S: TryInto<StatusCode>,
        S::Error: Debug,
        E: Into<BaseError>,
    {
        Self {
            status: status
                .try_into()
                .expect("Could not convert into a valid `StatusCode`"),
            error: error.into(),
            type_name: Some(std::any::type_name::<E>()),
        }
    }

    /// Create a new error object from static string.
    pub fn from_str<S, M>(status: S, msg: M) -> Self
    where
        S: TryInto<StatusCode>,
        S::Error: Debug,
        M: Display + Debug + Send + Sync + 'static,
    {
        Self {
            status: status
                .try_into()
                .expect("Could not convert into a valid `StatusCode`"),
            error: BaseError::msg(msg),
            type_name: None,
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
    pub fn set_status<S>(&mut self, status: S)
    where
        S: TryInto<StatusCode>,
        S::Error: Debug,
    {
        self.status = status
            .try_into()
            .expect("Could not convert into a valid `StatusCode`");
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
    ///
    /// Note: This function can be called whether or not backtraces
    /// are enabled and available. It will return a `None` variant if
    /// compiled on a toolchain that does not support backtraces, or
    /// if executed without backtraces enabled with
    /// `RUST_LIB_BACKTRACE=1`.
    #[cfg(all(backtrace, feature = "error_anyhow"))]
    pub fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        let backtrace = self.error.backtrace();
        if let std::backtrace::BacktraceStatus::Captured = backtrace.status() {
            Some(backtrace)
        } else {
            None
        }
    }

    #[cfg(all(not(backtrace), feature = "error_anyhow"))]
    #[allow(missing_docs)]
    pub fn backtrace(&self) -> Option<()> {
        None
    }

    #[cfg(all(backtrace, feature = "error_eyre"))]
    #[allow(missing_docs)]
    pub fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        self.error.backtrace()
    }

    #[cfg(all(not(backtrace), feature = "error_eyre"))]
    #[allow(missing_docs)]
    pub fn backtrace(&self) -> Option<&backtrace::Backtrace> {
        self.error.backtrace()
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

    /// Retrieves a reference to the type name of the error, if available.
    pub fn type_name(&self) -> Option<&str> {
        self.type_name.as_deref()
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.error, formatter)
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.error, formatter)
    }
}

impl<E: Into<BaseError>> From<E> for Error {
    fn from(error: E) -> Self {
        Self::new(StatusCode::InternalServerError, error)
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
