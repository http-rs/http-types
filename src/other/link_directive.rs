use std::fmt::{self, Display};
use std::str::FromStr;

use crate::bail_status as bail;

use super::RelationType;
use url::Url;

/// A value passed to the [`Link`][crate::other::Link] header.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinkDirective {
    url: Url,
    rel: Option<RelationType>,
    rev: Option<RelationType>,
    anchor: Option<String>,
}

impl Display for LinkDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl FromStr for LinkDirective {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(';');
        let url = match s.next() {
            Some(s) => match s.strip_prefix('<').map(|s| s.strip_suffix('>')).flatten() {
                Some(s) => Url::parse(s)?,
                None => bail!(
                    500,
                    "Expected a `LinkDirective` to contain a URL enclosed by a pair of brackets"
                ),
            },
            None => bail!(
                500,
                "Expected a `LinkDirective` to contain a URL enclosed by a pair of brackets"
            ),
        };
        todo!()
    }
}
