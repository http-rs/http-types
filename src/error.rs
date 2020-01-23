//! HTTP error types

use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};
use std::io;

use crate::StatusCode;

/// A specialized `Result` type for HTTP operations.
///
/// This type is broadly used across `http_types` for any operation which may
/// produce an error.
pub type Result<T> = std::result::Result<T, Error>;

/// A list specifying general categories of HTTP errors.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
///
/// It is used with the [`http_types::Error`] type.
///
/// [`http_types::Error`]: struct.Error.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ErrorKind {
    /// An entity was not found, often a file.
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// The connection was refused by the remote server.
    ConnectionRefused,
    /// The connection was reset by the remote server.
    ConnectionReset,
    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,
    /// The network operation failed because it was not connected yet.
    NotConnected,
    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    AddrInUse,
    /// A nonexistent interface was requested or the requested address was not
    /// local.
    AddrNotAvailable,
    /// The operation failed because a pipe was closed.
    BrokenPipe,
    /// An entity already exists, often a file.
    AlreadyExists,
    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    WouldBlock,
    /// A parameter was incorrect.
    InvalidInput,
    /// Data not valid for the operation were encountered.
    ///
    /// Unlike [`InvalidInput`], this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    ///
    /// [`InvalidInput`]: #variant.InvalidInput
    InvalidData,
    /// The I/O operation's timeout expired, causing it to be canceled.
    TimedOut,
    /// An error returned when an operation could not be completed because a
    /// call to [`write`] returned [`Ok(0)`].
    ///
    /// This typically means that an operation could only succeed if it wrote a
    /// particular number of bytes but only a smaller number of bytes could be
    /// written.
    ///
    /// [`write`]: ../../std/io/trait.Write.html#tymethod.write
    /// [`Ok(0)`]: ../../std/io/type.Result.html
    WriteZero,
    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    Interrupted,
    /// Any I/O error not part of this list.
    Other,

    /// An error returned when an operation could not be completed because an
    /// "end of file" was reached prematurely.
    ///
    /// This typically means that an operation could only succeed if it read a
    /// particular number of bytes but only a smaller number of bytes could be
    /// read.
    UnexpectedEof,
}

impl From<io::ErrorKind> for ErrorKind {
    fn from(kind: io::ErrorKind) -> Self {
        match kind {
            io::ErrorKind::NotFound => ErrorKind::NotFound,
            io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            io::ErrorKind::ConnectionRefused => ErrorKind::ConnectionRefused,
            io::ErrorKind::ConnectionReset => ErrorKind::ConnectionReset,
            io::ErrorKind::ConnectionAborted => ErrorKind::ConnectionAborted,
            io::ErrorKind::NotConnected => ErrorKind::NotConnected,
            io::ErrorKind::AddrInUse => ErrorKind::AddrInUse,
            io::ErrorKind::AddrNotAvailable => ErrorKind::AddrNotAvailable,
            io::ErrorKind::BrokenPipe => ErrorKind::BrokenPipe,
            io::ErrorKind::AlreadyExists => ErrorKind::AlreadyExists,
            io::ErrorKind::WouldBlock => ErrorKind::WouldBlock,
            io::ErrorKind::InvalidInput => ErrorKind::InvalidInput,
            io::ErrorKind::InvalidData => ErrorKind::InvalidData,
            io::ErrorKind::TimedOut => ErrorKind::TimedOut,
            io::ErrorKind::WriteZero => ErrorKind::WriteZero,
            io::ErrorKind::Interrupted => ErrorKind::Interrupted,
            io::ErrorKind::UnexpectedEof => ErrorKind::UnexpectedEof,
            io::ErrorKind::Other => ErrorKind::Other,
            _ => ErrorKind::Other,
        }
    }
}

impl From<ErrorKind> for io::Error {
    fn from(kind: ErrorKind) -> Self {
        let kind = match kind {
            ErrorKind::NotFound => io::ErrorKind::NotFound,
            ErrorKind::PermissionDenied => io::ErrorKind::PermissionDenied,
            ErrorKind::ConnectionRefused => io::ErrorKind::ConnectionRefused,
            ErrorKind::ConnectionReset => io::ErrorKind::ConnectionReset,
            ErrorKind::ConnectionAborted => io::ErrorKind::ConnectionAborted,
            ErrorKind::NotConnected => io::ErrorKind::NotConnected,
            ErrorKind::AddrInUse => io::ErrorKind::AddrInUse,
            ErrorKind::AddrNotAvailable => io::ErrorKind::AddrNotAvailable,
            ErrorKind::BrokenPipe => io::ErrorKind::BrokenPipe,
            ErrorKind::AlreadyExists => io::ErrorKind::AlreadyExists,
            ErrorKind::WouldBlock => io::ErrorKind::WouldBlock,
            ErrorKind::InvalidInput => io::ErrorKind::InvalidInput,
            ErrorKind::InvalidData => io::ErrorKind::InvalidData,
            ErrorKind::TimedOut => io::ErrorKind::TimedOut,
            ErrorKind::WriteZero => io::ErrorKind::WriteZero,
            ErrorKind::Interrupted => io::ErrorKind::Interrupted,
            ErrorKind::UnexpectedEof => io::ErrorKind::UnexpectedEof,
            ErrorKind::Other => io::ErrorKind::Other,
        };
        io::Error::from(kind)
    }
}

/// Internal representation of the error state.
#[derive(Debug)]
enum Repr {
    Io(io::Error),
    Custom(anyhow::Error),
}

/// The error type for HTTP operations.
pub struct Error {
    repr: Repr,
    kind: ErrorKind,
    status: crate::StatusCode,
}

impl Error {
    /// Create a new error object from any error type.
    ///
    /// The error type must be threadsafe and 'static, so that the Error will be
    /// as well. If the error type does not provide a backtrace, a backtrace will
    /// be created here to ensure that a backtrace exists.
    pub fn new<E>(kind: ErrorKind, error: E, status: StatusCode) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        let error = anyhow::Error::new(error);
        Self {
            kind,
            repr: Repr::Custom(error),
            status,
        }
    }

    /// Create a new error from a message.
    pub(crate) fn new_adhoc<M>(message: M) -> Error
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        Error {
            kind: ErrorKind::Other,
            repr: Repr::Custom(anyhow::format_err!(message)),
            status: StatusCode::InternalServerError,
        }
    }

    /// Create a new error object from an I/O error.
    pub fn from_io(error: std::io::Error, status: StatusCode) -> Self {
        Self {
            kind: error.kind().into(),
            repr: Repr::Io(error),
            status,
        }
    }

    /// Get the status code associated with this error.
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Set the status code associated with this error.
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }

    /// Returns the corresponding ErrorKind for this error.
    pub fn kind(&self) -> ErrorKind {
        self.kind.clone()
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
        match self {
            Repr::Io => std::backtrace::Backtrace::capture(),
            Repr::Custom(err) => err.backtrace(),
        }
    }

    /// Attempt to downcast the error object to a concrete type.
    pub fn downcast<E>(self) -> std::result::Result<E, Self>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        match self.repr {
            Repr::Io(err) => Ok(anyhow::Error::new(err).downcast().unwrap()),
            Repr::Custom(err) if err.downcast_ref::<E>().is_some() => Ok(err.downcast().unwrap()),
            _ => Err(self),
        }
    }

    /// Downcast this error object by reference.
    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        match self.repr {
            Repr::Custom(ref err) => err.downcast_ref::<E>(),
            _ => None,
        }
    }

    /// Downcast this error object by mutable reference.
    pub fn downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        match self.repr {
            Repr::Custom(ref mut err) => err.downcast_mut::<E>(),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.repr {
            Repr::Io(io) => write!(formatter, "{}", io),
            Repr::Custom(err) => write!(formatter, "{}", err),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.repr {
            Repr::Io(io) => write!(formatter, "{}", io),
            Repr::Custom(err) => write!(formatter, "{}", err),
        }
    }
}

impl<E> From<E> for Error
where
    E: StdError + Send + Sync + 'static,
{
    fn from(error: E) -> Self {
        Self {
            kind: ErrorKind::Other,
            repr: Repr::Custom(anyhow::Error::new(error)),
            status: StatusCode::InternalServerError,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            kind: kind.clone(),
            repr: Repr::Io(kind.into()),
            status: StatusCode::InternalServerError,
        }
    }
}

impl AsRef<dyn StdError + Send + Sync> for Error {
    fn as_ref(&self) -> &(dyn StdError + Send + Sync + 'static) {
        match &self.repr {
            Repr::Io(ref io) => io,
            Repr::Custom(ref err) => err.as_ref(),
        }
    }
}

impl AsRef<dyn StdError> for Error {
    fn as_ref(&self) -> &(dyn StdError + 'static) {
        match &self.repr {
            Repr::Io(ref io) => io,
            Repr::Custom(ref err) => err.as_ref(),
        }
    }
}

impl From<Error> for Box<dyn StdError + Send + Sync + 'static> {
    fn from(error: Error) -> Self {
        match error.repr {
            Repr::Io(io) => io.into(),
            Repr::Custom(err) => err.into(),
        }
    }
}

impl From<Error> for Box<dyn StdError + 'static> {
    fn from(error: Error) -> Self {
        Box::<dyn StdError + Send + Sync>::from(error)
    }
}
