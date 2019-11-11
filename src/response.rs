use crate::Headers;

/// An HTTP response.
#[derive(Debug)]
pub struct Response {
    headers: Headers,
}
