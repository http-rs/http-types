use crate::headers::HeaderValue;
use std::fmt::{self, Display};

/// A list of `HeaderValue`s.
///
/// This always contains at least one header value.
#[derive(Debug)]
pub struct HeaderValues {
    inner: Vec<HeaderValue>,
}

impl Display for HeaderValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for v in &self.inner {
            list.entry(&v);
        }
        list.finish()
    }
}

impl PartialEq<str> for HeaderValues {
    fn eq(&self, other: &str) -> bool {
        self.inner[0] == other
    }
}

impl<'a> PartialEq<&'a str> for HeaderValues {
    fn eq(&self, other: &&'a str) -> bool {
        &self.inner[0] == other
    }
}

impl PartialEq<String> for HeaderValues {
    fn eq(&self, other: &String) -> bool {
        &self.inner[0] == other
    }
}

impl<'a> PartialEq<&String> for HeaderValues {
    fn eq(&self, other: &&String) -> bool {
        &&self.inner[0] == other
    }
}
