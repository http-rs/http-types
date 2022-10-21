use core::convert::{Infallible, TryInto};
use std::error::Error as StdError;

use crate::{ResponseError, StatusCode};

/// Provides the `status` method for `Result` and `Option`.
///
/// This trait is sealed and cannot be implemented outside of `http-types`.
pub trait Status<T, E>: private::Sealed {
    /// Wrap the error value with an additional status code.
    fn status<S>(self, status: S) -> Result<T, ResponseError>
    where
        S: TryInto<StatusCode>,
        S::Error: StdError + Send + Sync + 'static;

    /// Wrap the error value with an additional status code that is evaluated
    /// lazily only once an error does occur.
    fn with_status<S, F>(self, f: F) -> Result<T, ResponseError>
    where
        S: TryInto<StatusCode>,
        S::Error: StdError + Send + Sync + 'static,
        F: FnOnce() -> S;
}

impl<T, E> Status<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    /// Wrap the error value with an additional status code.
    ///
    /// # Panics
    ///
    /// Panics if [`Status`][status] is not a valid [`StatusCode`][statuscode].
    ///
    /// [status]: crate::Status
    /// [statuscode]: crate::StatusCode
    fn status<S>(self, status: S) -> Result<T, ResponseError>
    where
        S: TryInto<StatusCode>,
        S::Error: StdError + Send + Sync + 'static,
    {
        self.map_err(|error| ResponseError::new_status(status, error))
    }

    /// Wrap the error value with an additional status code that is evaluated
    /// lazily only once an error does occur.
    ///
    /// # Panics
    ///
    /// Panics if [`Status`][status] is not a valid [`StatusCode`][statuscode].
    ///
    /// [status]: crate::Status
    /// [statuscode]: crate::StatusCode
    fn with_status<S, F>(self, f: F) -> Result<T, ResponseError>
    where
        S: TryInto<StatusCode>,
        S::Error: StdError + Send + Sync + 'static,
        F: FnOnce() -> S,
    {
        self.map_err(|error| ResponseError::new_status(f(), error))
    }
}

impl<T> Status<T, Infallible> for Option<T> {
    /// Wrap the error value with an additional status code.
    ///
    /// # Panics
    ///
    /// Panics if [`Status`][status] is not a valid [`StatusCode`][statuscode].
    ///
    /// [status]: crate::Status
    /// [statuscode]: crate::StatusCode
    fn status<S>(self, status: S) -> Result<T, ResponseError>
    where
        S: TryInto<StatusCode>,
        S::Error: StdError + Send + Sync + 'static,
    {
        self.ok_or_else(|| ResponseError::from_str_status(status, "NoneError"))
    }

    /// Wrap the error value with an additional status code that is evaluated
    /// lazily only once an error does occur.
    ///
    /// # Panics
    ///
    /// Panics if [`Status`][status] is not a valid [`StatusCode`][statuscode].
    ///
    /// [status]: crate::Status
    /// [statuscode]: crate::StatusCode
    fn with_status<S, F>(self, f: F) -> Result<T, ResponseError>
    where
        S: TryInto<StatusCode>,
        S::Error: StdError + Send + Sync + 'static,
        F: FnOnce() -> S,
    {
        self.ok_or_else(|| ResponseError::from_str_status(f(), "NoneError"))
    }
}

pub(crate) mod private {
    pub trait Sealed {}

    impl<T, E> Sealed for Result<T, E> {}
    impl<T> Sealed for Option<T> {}
}

#[cfg(test)]
mod test {
    use super::Status;

    #[test]
    fn construct_shorthand_with_valid_status_code() {
        Some(()).status(200).unwrap();
    }

    #[test]
    #[should_panic(expected = "Could not convert into a valid `StatusCode`")]
    fn construct_shorthand_with_invalid_status_code() {
        let res: Result<(), std::io::Error> =
            Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"));
        res.status(600).unwrap();
    }
}
