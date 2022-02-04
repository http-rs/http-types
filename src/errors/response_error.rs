use std::convert::TryInto;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};

use anyhow::Context;

use crate::StatusCode;

/// An error type to be used for where handlers and middleware can error when handling an http response.
#[derive(Debug)]
pub struct ResponseError {
    pub(super) error: anyhow::Error,
    status: Option<crate::StatusCode>,
    type_name: Option<&'static str>,
}

#[allow(unreachable_pub)]
#[derive(Debug)]
#[doc(hidden)]
pub struct BacktracePlaceholder;

impl Display for BacktracePlaceholder {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!()
    }
}

impl ResponseError {
    /// Create a new error object from any error type.
    ///
    /// The error type must be thread-safe and 'static, so that the Error will be
    /// as well. If the error type does not provide a backtrace, a backtrace will
    /// be created here to ensure that a backtrace exists.
    pub fn new<E>(error: E) -> Self
    where
        E: Into<anyhow::Error>,
    {
        Self {
            status: None,
            error: error.into(),
            type_name: Some(std::any::type_name::<E>()),
        }
    }

    /// Create a new error object from any error type.
    ///
    /// The error type must be thread-safe and 'static, so that the Error will be
    /// as well. If the error type does not provide a backtrace, a backtrace will
    /// be created here to ensure that a backtrace exists.
    pub fn new_status<S, E>(status: S, error: E) -> Self
    where
        S: TryInto<StatusCode>,
        S::Error: StdError + Send + Sync + 'static,
        E: Into<anyhow::Error>,
    {
        let mut err = Self::new(error);
        if let Err(new_err) = err.set_status(status) {
            return new_err;
        }
        err
    }

    /// Create a new error object from static string.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<M>(msg: M) -> Self
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        Self {
            status: None,
            error: anyhow::Error::msg(msg),
            type_name: None,
        }
    }

    /// Create a new error object from static string.
    pub fn from_str_status<S, M>(status: S, msg: M) -> Self
    where
        S: TryInto<StatusCode>,
        S::Error: StdError + Send + Sync + 'static,
        M: Display + Debug + Send + Sync + 'static,
    {
        let mut err = Self::from_str(msg);
        if let Err(new_err) = err.set_status(status) {
            return new_err;
        }
        err
    }

    /// Get the status code associated with this error.
    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }

    /// Set the status code associated with this error.
    pub fn set_status<S>(&mut self, status: S) -> Result<(), ResponseError>
    where
        S: TryInto<StatusCode>,
        S::Error: StdError + Send + Sync + 'static,
    {
        self.status = Some(
            status
                .try_into()
                .context("Could not convert into a valid `StatusCode`")?,
        );
        Ok(())
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
    #[cfg(backtrace)]
    pub fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        let backtrace = self.error.backtrace();
        if let std::backtrace::BacktraceStatus::Captured = backtrace.status() {
            Some(backtrace)
        } else {
            None
        }
    }

    #[cfg(not(backtrace))]
    #[allow(missing_docs)]
    pub const fn backtrace(&self) -> Option<BacktracePlaceholder> {
        None
    }

    /// Returns the inner [`anyhow::Error`]
    /// Note: This will lose status code information
    pub fn into_inner(self) -> anyhow::Error {
        self.error
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
        self.type_name
    }

    /// Converts anything which implements `Display` into an `http_types::Error`.
    ///
    /// This is handy for errors which are not `Send + Sync + 'static` because `std::error::Error` requires `Display`.
    /// Note that any associated context not included in the `Display` output will be lost,
    /// and so this may be lossy for some types which implement `std::error::Error`.
    ///
    /// **Note: Prefer `error.into()` via `From<Into<anyhow::Error>>` when possible!**
    pub fn from_display<D: Display>(error: D) -> Self {
        anyhow::Error::msg(error.to_string()).into()
    }

    /// Converts anything which implements `Debug` into an `http_types::Error`.
    ///
    /// This is handy for errors which are not `Send + Sync + 'static` because `std::error::Error` requires `Debug`.
    /// Note that any associated context not included in the `Debug` output will be lost,
    /// and so this may be lossy for some types which implement `std::error::Error`.
    ///
    /// **Note: Prefer `error.into()` via `From<Into<anyhow::Error>>` when possible!**
    pub fn from_debug<D: Debug>(error: D) -> Self {
        anyhow::Error::msg(format!("{:?}", error)).into()
    }
}

impl Display for ResponseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.error, formatter)
    }
}

impl<E: Into<anyhow::Error>> From<E> for ResponseError {
    fn from(error: E) -> Self {
        Self::new(error)
    }
}

impl AsRef<dyn StdError + Send + Sync> for ResponseError {
    fn as_ref(&self) -> &(dyn StdError + Send + Sync + 'static) {
        self.error.as_ref()
    }
}

impl AsRef<dyn StdError> for ResponseError {
    fn as_ref(&self) -> &(dyn StdError + 'static) {
        self.error.as_ref()
    }
}

impl From<ResponseError> for Box<dyn StdError + Send + Sync + 'static> {
    fn from(error: ResponseError) -> Self {
        error.error.into()
    }
}

impl From<ResponseError> for Box<dyn StdError + 'static> {
    fn from(error: ResponseError) -> Self {
        Box::<dyn StdError + Send + Sync>::from(error.error)
    }
}

impl AsRef<anyhow::Error> for ResponseError {
    fn as_ref(&self) -> &anyhow::Error {
        &self.error
    }
}
