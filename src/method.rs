use std::fmt::{self, Display};

/// HTTP request methods.
///
/// [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods)
#[derive(Debug, Clone, Copy)]
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

impl<'a> std::convert::TryFrom<&'a str> for Method {
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => unimplemented!(),
        })
    }
}
