mod date;

pub(crate) use date::fmt_http_date;
pub(crate) use date::parse_http_date;

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
