use crate::{Error, StatusCode};
use core::convert::Infallible;
use std::error::Error as StdError;

/// Provides the `status` method for `Result`.
///
/// This trait is sealed and cannot be implemented outside of `http-types`.
pub trait ResultExt<T, E>: private::Sealed {
    /// Wrap the error value with an additional status code.
    fn status(self, status: StatusCode) -> Result<T, Error>;

    /// Wrap the error value with an additional status code that is evaluated
    /// lazily only once an error does occur.
    fn with_status<F>(self, f: F) -> Result<T, Error>
    where
        F: FnOnce() -> StatusCode;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn status(self, status: StatusCode) -> Result<T, Error> {
        self.map_err(|error| Error::new(status, error))
    }

    fn with_status<F>(self, f: F) -> Result<T, Error>
    where
        F: FnOnce() -> StatusCode,
    {
        self.map_err(|error| Error::new(f(), error))
    }
}

impl<T> ResultExt<T, Infallible> for Option<T> {
    fn status(self, status: StatusCode) -> Result<T, Error> {
        self.ok_or_else(|| Error::from_str(status, "NoneError"))
    }

    fn with_status<F>(self, f: F) -> Result<T, Error>
    where
        F: FnOnce() -> StatusCode,
    {
        self.ok_or_else(|| Error::from_str(f(), "NoneError"))
    }
}

pub(crate) mod private {
    pub trait Sealed {}

    impl<T, E> Sealed for Result<T, E> {}
    impl<T> Sealed for Option<T> {}
}
