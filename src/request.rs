use crate::{Headers, Method, Url};

/// An HTTP request.
#[derive(Debug)]
pub struct Request {
    method: Method,
    url: Url,
    headers: Headers,
}

impl Request {
    /// Create a new request.
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: Headers::new(),
        }
    }
}
