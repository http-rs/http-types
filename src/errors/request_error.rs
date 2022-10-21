use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};

use miette::Diagnostic;
use thiserror::Error as ThisError;

use crate::StatusCode;

use super::{Error, ResponseError};

#[derive(Debug, Diagnostic, ThisError)]
/// An error type to be used for clients which handle http requests.
pub enum RequestError {
    #[error(transparent)]
    /// An internal, concrete http-types error without indirection.
    Internal(Error),
    #[error(transparent)]
    /// A dynamic error, usually generated in a response handler.
    ///
    /// This has a layer of indirection to get around trait conflicts regarding StdErr and anyhow.
    Dynamic(ResponseErrorIndirection),
}

pub struct ResponseErrorIndirection(ResponseError);

impl StdError for ResponseErrorIndirection {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.error.source()
    }

    #[cfg(backtrace)]
    fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        Some(self.0.error.backtrace())
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn StdError> {
        self.source()
    }
}

impl RequestError {
    /// Get the status code associated with this error.
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            RequestError::Internal(inner) => inner.associated_status_code(),
            RequestError::Dynamic(ResponseErrorIndirection(inner)) => inner.status(),
        }
    }
}

impl Debug for ResponseErrorIndirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl Display for ResponseErrorIndirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
