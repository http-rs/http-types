use crate::{Error, StatusCode};
use core::convert::Infallible;
use std::error::Error as StdError;

/// Provides the `status` method for `Result`.
///
/// This trait is sealed and cannot be implemented outside of `http-types`.
pub trait ResultExt<T, E>: private::Sealed {
    /// Wrap the error value with an additional status code.
    fn status<S>(self, status: S) -> Result<T, Error>
    where
        S: Into<StatusCode>;

    /// Wrap the error value with an additional status code that is evaluated
    /// lazily only once an error does occur.
    fn with_status<S, F>(self, f: F) -> Result<T, Error>
    where
        S: Into<StatusCode>,
        F: FnOnce() -> S;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn status<S>(self, status: S) -> Result<T, Error> where
    S: Into<StatusCode> {
        self.map_err(|error| Error::new(status.into(), error))
    }

    fn with_status<S, F>(self, f: F) -> Result<T, Error>
    where
        S: Into<StatusCode>,
        F: FnOnce() -> S,
    {
        self.map_err(|error| Error::new(f().into(), error))
    }
}

impl<T> ResultExt<T, Infallible> for Option<T> {
    fn status<S>(self, status: S) -> Result<T, Error> 
    where S: Into<StatusCode> {
        self.ok_or_else(|| Error::from_str(status.into(), "NoneError"))
    }

    fn with_status<S, F>(self, f: F) -> Result<T, Error>
    where
        S: Into<StatusCode>,
        F: FnOnce() -> S,
    {
        self.ok_or_else(|| Error::from_str(f().into(), "NoneError"))
    }
}

pub(crate) mod private {
    pub trait Sealed {}

    impl<T, E> Sealed for Result<T, E> {}
    impl<T> Sealed for Option<T> {}
}
