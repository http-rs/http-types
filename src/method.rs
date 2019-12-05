use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

/// HTTP request methods.
///
/// [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Method {
    /// The GET method requests a representation of the specified resource. Requests using GET
    /// should only retrieve data.
    Get,

    /// The HEAD method asks for a response identical to that of a GET request, but without the response body.
    Head,

    /// The POST method is used to submit an entity to the specified resource, often causing a
    /// change in state or side effects on the server.
    Post,

    /// The PUT method replaces all current representations of the target resource with the request
    /// payload.
    Put,

    /// The DELETE method deletes the specified resource.
    Delete,

    /// The CONNECT method establishes a tunnel to the server identified by the target resource.
    Connect,

    /// The OPTIONS method is used to describe the communication options for the target resource.
    Options,

    /// The TRACE method performs a message loop-back test along the path to the target resource.
    Trace,

    /// The PATCH method is used to apply partial modifications to a resource.
    Patch,
}

impl Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Head => write!(f, "HEAD"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            Self::Connect => write!(f, "CONNECT"),
            Self::Options => write!(f, "OPTIONS"),
            Self::Trace => write!(f, "TRACE"),
            Self::Patch => write!(f, "PATCH"),
        }
    }
}

/// An error returned when failing to convert into a status code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseError {
    _private: (),
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Error parsing a string into a status code".fmt(f)
    }
}

impl FromStr for Method {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "PATCH" => Ok(Self::Patch),
            _ => Err(ParseError { _private: () }),
        }
    }
}

impl<'a> TryFrom<&'a str> for Method {
    type Error = ParseError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse()
    }
}
