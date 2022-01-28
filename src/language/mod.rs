//! RFC 4647 Language Ranges.
//! 
//! [Read more](https://datatracker.ietf.org/doc/html/rfc4647)

mod parse;

use crate::headers::HeaderValue;
use std::{fmt::{self, Display}, borrow::Cow, str::FromStr};

#[derive(Debug)]
pub struct LanguageRange {
    pub(crate) tags: Vec<Cow<'static, str>>
}

impl Display for LanguageRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tags = self.tags.iter();
        if let Some(tag) = tags.next() {
            write!(f, "{}", tag)?;

            for tag in tags {
                write!(f, "-{}", tag)?;
            }
        }
        Ok(())
    }
}

impl From<LanguageRange> for HeaderValue {
    fn from(language: LanguageRange) -> Self {
        let s = language.to_string();
        unsafe { HeaderValue::from_bytes_unchecked(s.into_bytes()) }
    }
}

impl FromStr for LanguageRange {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::parse(s)
    }
}
