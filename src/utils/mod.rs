mod date;

pub(crate) use date::fmt_http_date;
pub(crate) use date::parse_http_date;

use crate::{Error, Status, StatusCode};
use std::str::FromStr;

/// Declares unstable items.
#[doc(hidden)]
macro_rules! cfg_unstable {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "unstable")]
            #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
            $item
        )*
    }
}

/// Parse a weight of the form `q=0.123`.
pub(crate) fn parse_weight(s: &str) -> crate::Result<f32> {
    let mut parts = s.split("=");
    if !matches!(parts.next(), Some("q")) {
        let mut err = Error::new_adhoc("invalid weight");
        err.set_status(StatusCode::BadRequest);
        return Err(err);
    }
    match parts.next() {
        Some(s) => {
            let weight = f32::from_str(s).status(400)?;
            Ok(weight)
        }
        None => {
            let mut err = Error::new_adhoc("invalid weight");
            err.set_status(StatusCode::BadRequest);
            Err(err)
        }
    }
}
