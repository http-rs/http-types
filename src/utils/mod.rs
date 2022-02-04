mod date;

pub(crate) use date::fmt_http_date;
pub(crate) use date::parse_http_date;
pub(crate) use date::HttpDate;

use crate::errors::HeaderError;

use std::cmp::Ordering;
use std::str::FromStr;

/// Parse a weight of the form `q=0.123`.
pub(crate) fn parse_weight(s: &str) -> crate::Result<f32> {
    let mut parts = s.split('=');
    if !matches!(parts.next(), Some("q")) {
        return Err(HeaderError::SpecificityInvalid.into());
    }
    match parts.next() {
        Some(s) => {
            let weight = f32::from_str(s).map_err(|_| HeaderError::SpecificityInvalid)?;
            Ok(weight)
        }
        None => Err(HeaderError::SpecificityInvalid.into()),
    }
}

/// Order proposals by weight. Try ordering by q value first. If equal or undefined,
/// order by index, favoring the latest provided value.
pub(crate) fn sort_by_weight<T: PartialOrd + Clone>(props: &mut Vec<T>) {
    let mut arr: Vec<(usize, T)> = props.iter().cloned().enumerate().collect();
    arr.sort_unstable_by(|a, b| match b.1.partial_cmp(&a.1) {
        None | Some(Ordering::Equal) => b.0.cmp(&a.0),
        Some(ord) => ord,
    });
    *props = arr.into_iter().map(|(_, t)| t).collect::<Vec<T>>();
}
