use crate::{Error, StatusCode};

use std::fmt::{self, Debug, Display};
use std::str::FromStr;

pub(crate) const ACCEPT_RANGE_VALUE: &str = "bytes";
pub(crate) const RANGE_PREFIX: &str = "bytes=";
pub(crate) const CONTENT_RANGE_PREFIX: &str = "bytes ";

/// The representation of a single HTTP byte range.
///
/// # Specifications
///
/// - [RFC 7233, section 2.1: Range](https://tools.ietf.org/html/rfc7233#section-2.1)
/// - [RFC 7233, Appendix D: Collected ABNF](https://tools.ietf.org/html/rfc7233#appendix-D)
#[derive(Default, Debug, Clone, Eq, PartialEq, Copy)]
pub struct BytesRange {
    /// The range start.
    ///
    /// If empty the ends indicates a relative start
    /// from the end of the document.
    pub start: Option<u64>,
    /// The range end.
    ///
    /// If empty the range goes through the end
    /// of the document.
    pub end: Option<u64>,
}

impl BytesRange {
    /// Create a new instance with start and end.
    pub fn new<S, E>(start: S, end: E) -> Self
    where
        S: Into<Option<u64>>,
        E: Into<Option<u64>>,
    {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    /// Returns true if the range's bounds match the given document size.
    pub fn match_size(&self, size: u64) -> bool {
        if let Some(start) = self.start {
            if start > size - 1 {
                return false;
            }
        }
        if let Some(end) = self.end {
            if end > size - 1 {
                return false;
            }
        }
        true
    }
}

impl Display for BytesRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}",
            self.start
                .map(|v| v.to_string())
                .unwrap_or_else(String::new),
            self.end.map(|v| v.to_string()).unwrap_or_else(String::new),
        )
    }
}

impl FromStr for BytesRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fn_err = || {
            Err(Error::from_str(
                StatusCode::RequestedRangeNotSatisfiable,
                "Invalid Range header for byte ranges",
            ))
        };

        let mut s = s.trim().splitn(2, '-');
        let start = str_to_bound(s.next())?;
        let end = str_to_bound(s.next())?;

        if start.is_none() && end.is_none() {
            return fn_err();
        }

        if let Some(start) = start {
            if let Some(end) = end {
                if end <= start {
                    return fn_err();
                }
            }
        }

        Ok(BytesRange::new(start, end))
    }
}

fn str_to_bound(s: Option<&str>) -> crate::Result<Option<u64>> {
    s.and_then(|s| if s.is_empty() { None } else { Some(s) })
        .map(|s| {
            u64::from_str(s).map_err(|_| {
                Error::from_str(
                    StatusCode::RequestedRangeNotSatisfiable,
                    "Invalid Range header for byte ranges",
                )
            })
        })
        .transpose()
}

/// A set of `BytesRange` representing a range request.
///
///
/// # Specifications
///
/// - [RFC 7233, section 3.1: Range](https://tools.ietf.org/html/rfc7233#section-3.1)
/// - [RFC 7233, Appendix D: Collected ABNF](https://tools.ietf.org/html/rfc7233#appendix-D)
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct BytesRangeSet {
    ranges: Vec<BytesRange>,
}

impl BytesRangeSet {
    /// Create a new instance with an empty range set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a new byte range at the end of the byte range set.
    pub fn push<S, E>(&mut self, start: S, end: E)
    where
        S: Into<Option<u64>>,
        E: Into<Option<u64>>,
    {
        let range = BytesRange::new(start, end);
        self.ranges.push(range);
    }

    /// Pushes a `BytesRange` at the end of the byte range set.
    pub fn push_range<S, E>(&mut self, range: BytesRange)
    where
        S: Into<Option<u64>>,
        E: Into<Option<u64>>,
    {
        self.ranges.push(range);
    }

    /// Returns the number of `BytesRange` in the set.
    pub fn len(&self) -> usize {
        self.ranges.len()
    }

    /// Returns true if the set contains no element.
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// Returns an `Iterator` over the `BytesRange`.
    pub fn iter(&self) -> impl Iterator<Item = &BytesRange> {
        self.ranges.iter()
    }

    /// Returns the first `BytesRange` in the set.
    ///
    /// # Panics
    ///
    /// Panics if the set is empty. A `BytesRangeSet`
    /// built from a `Range` header is warranted to be none empty.
    pub fn first(&self) -> Option<BytesRange> {
        self.ranges.get(0).copied()
    }

    /// Validates that the ranges are within the expected document size.
    ///
    /// Returns `HTTP 416 Range Not Satisfiable` if one range is out of bounds.
    ///
    /// # Examples
    ///
    /// Most of the time applications want to validate the range set against
    /// the actual size of the document, as per the RFC specification:
    ///
    /// ```
    /// # fn main() -> http_types::Result<()> {
    /// #
    /// use http_types::range::{BytesRange, BytesRangeSet, Range};
    /// use http_types::{Method, Request, StatusCode, Url};
    /// use std::convert::TryInto;
    ///
    /// let mut range_set = BytesRangeSet::new();
    /// range_set.push(0, 500);
    /// let range = Range::Bytes(range_set);
    ///
    /// let mut req = Request::new(Method::Get, Url::parse("http://example.com").unwrap());
    /// range.apply(&mut req);
    ///
    /// let range = Range::from_headers(req)?.unwrap();
    ///
    /// if let Range::Bytes(range_set) = range {
    ///     let err = range_set.match_size(350).unwrap_err();
    ///     assert_eq!(err.status(), StatusCode::RequestedRangeNotSatisfiable);
    /// }
    /// #
    /// # Ok(()) }
    /// ```
    pub fn match_size(&self, size: u64) -> crate::Result<()> {
        for range in &self.ranges {
            if !range.match_size(size) {
                return Err(Error::from_str(
                    StatusCode::RequestedRangeNotSatisfiable,
                    "Invalid Range header for byte ranges",
                ));
            }
        }
        Ok(())
    }

    /// Create a ByteRanges from a string.
    pub(crate) fn from_str(s: &str) -> crate::Result<Self> {
        let fn_err = || {
            Error::from_str(
                StatusCode::BadRequest,
                "Invalid Range header for byte ranges",
            )
        };

        let mut ranges = Self::new();

        for range_str in s.split(',') {
            let range = BytesRange::from_str(range_str)?;
            ranges.ranges.push(range);
        }

        if ranges.ranges.is_empty() {
            return Err(fn_err());
        }

        Ok(ranges)
    }
}

impl IntoIterator for BytesRangeSet {
    type Item = BytesRange;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into_iter()
    }
}

impl<'a> IntoIterator for &'a BytesRangeSet {
    type Item = &'a BytesRange;
    type IntoIter = std::slice::Iter<'a, BytesRange>;

    fn into_iter(self) -> Self::IntoIter {
        self.ranges.iter()
    }
}

impl Display for BytesRangeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, range) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", range)?;
        }
        Ok(())
    }
}

/// The representation of a HTTP ContentRange response header with bytes.
///
/// # Specifications
///
/// - [RFC 7233, section 4.2: Range](https://tools.ietf.org/html/rfc7233#section-4.2)
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct BytesContentRange {
    size: Option<u64>,
    range: Option<BytesRange>,
}

impl BytesContentRange {
    /// Create a new instance with no range and no size.
    pub fn new() -> Self {
        BytesContentRange::default()
    }

    /// Returns a new instance with a given range defined by `start` and `end` bounds.
    pub fn with_range(mut self, start: u64, end: u64) -> Self {
        self.range = Some(BytesRange::new(start, end));
        self
    }

    /// Returns a new instance with a size.
    pub fn with_size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// Returns the `ByteRange` if any.
    pub fn range(&self) -> Option<BytesRange> {
        self.range
    }

    /// Returns the size if any.
    pub fn size(&self) -> Option<u64> {
        self.size
    }

    /// Create a ByteRanges from a string.
    pub(crate) fn from_str(s: &str) -> crate::Result<Self> {
        let fn_err = || {
            Error::from_str(
                StatusCode::RequestedRangeNotSatisfiable,
                "Invalid Content-Range value",
            )
        };

        let mut bytes_content_range = BytesContentRange::new();

        let mut s = s.trim_start().splitn(2, '/');

        let range_s = s.next().ok_or_else(fn_err)?;
        if range_s != "*" {
            let range = BytesRange::from_str(range_s).map_err(|_| fn_err())?;
            if range.start.is_none() || range.end.is_none() {
                return Err(fn_err());
            }
            bytes_content_range.range.replace(range);
        }

        let size_s = s.next().ok_or_else(fn_err)?;
        if size_s != "*" {
            let size = u64::from_str(size_s).map_err(|_| fn_err())?;
            bytes_content_range = bytes_content_range.with_size(size);
        }

        if bytes_content_range.range.is_none() && bytes_content_range.size.is_none() {
            return Err(fn_err());
        }
        if let Some(size) = bytes_content_range.size {
            if let Some(end) = bytes_content_range.range.and_then(|r| r.end) {
                if size <= end {
                    return Err(fn_err());
                }
            }
        }

        Ok(bytes_content_range)
    }
}

impl Display for BytesContentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}",
            self.range
                .map(|r| r.to_string())
                .unwrap_or_else(|| "*".into()),
            self.size
                .map(|s| s.to_string())
                .unwrap_or_else(|| "*".into())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_range_start_end() -> crate::Result<()> {
        let range = BytesRange::from_str("1-5")?;
        assert_eq!(range, BytesRange::new(1, 5));
        Ok(())
    }

    #[test]
    fn byte_range_start_no_end() -> crate::Result<()> {
        let range = BytesRange::from_str("1-")?;
        assert_eq!(range, BytesRange::new(1, None));
        Ok(())
    }

    #[test]
    fn byte_range_no_start_end() -> crate::Result<()> {
        let range = BytesRange::from_str("-5")?;
        assert_eq!(range, BytesRange::new(None, 5));
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Invalid Range header for byte ranges")]
    fn byte_range_no_start_no_end() {
        BytesRange::from_str("-").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid Range header for byte ranges")]
    fn byte_range_start_after_end() {
        BytesRange::from_str("3-1").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid Range header for byte ranges")]
    fn byte_range_invalid_integer() {
        BytesRange::from_str("abc-5").unwrap();
    }

    #[test]
    fn byte_range_match_size() {
        let range = BytesRange::new(0, 4);
        assert_eq!(range.match_size(5), true);
    }

    #[test]
    fn byte_range_not_match_size() {
        let range = BytesRange::new(0, 4);
        assert_eq!(range.match_size(3), false);
    }

    #[test]
    fn byte_range_not_match_size_start() {
        let range = BytesRange::new(4, None);
        assert_eq!(range.match_size(4), false);
    }

    #[test]
    fn byte_range_not_match_size_end() {
        let range = BytesRange::new(None, 5);
        assert_eq!(range.match_size(5), false);
    }

    #[test]
    fn bytes_range_set_single_range() -> crate::Result<()> {
        let range_set = BytesRangeSet::from_str("1-5")?;
        assert_eq!(range_set.len(), 1);
        assert_eq!(range_set.first(), Some(BytesRange::new(1, 5)));
        Ok(())
    }

    #[test]
    fn bytes_range_set_multiple_ranges() -> crate::Result<()> {
        let range_set = BytesRangeSet::from_str("1-5, -5")?;
        assert_eq!(range_set.len(), 2);
        let mut iter = range_set.iter();
        assert_eq!(iter.next(), Some(&BytesRange::new(1, 5)));
        assert_eq!(iter.next(), Some(&BytesRange::new(None, 5)));
        Ok(())
    }

    #[test]
    fn bytes_range_set_no_match_size() {
        let range_set = BytesRangeSet::from_str("1-5, -10").unwrap();
        let err = range_set.match_size(6).unwrap_err();
        assert_eq!(err.status(), StatusCode::RequestedRangeNotSatisfiable);
    }

    #[test]
    fn bytes_range_match_size() {
        let range_set = BytesRangeSet::from_str("1-5, -10").unwrap();
        range_set.match_size(11).unwrap();
    }

    #[test]
    fn bytes_content_range_and_size() -> crate::Result<()> {
        let content_range = BytesContentRange::from_str("1-5/100")?;
        assert_eq!(content_range.range(), Some(BytesRange::new(1, 5)));
        assert_eq!(content_range.size(), Some(100));
        Ok(())
    }

    #[test]
    fn bytes_content_range_and_unknown_size() -> crate::Result<()> {
        let content_range = BytesContentRange::from_str("1-5/*")?;
        assert_eq!(content_range.range(), Some(BytesRange::new(1, 5)));
        assert_eq!(content_range.size(), None);
        Ok(())
    }

    #[test]
    fn bytes_content_no_range_and_size() -> crate::Result<()> {
        let content_range = BytesContentRange::from_str("*/100")?;
        assert_eq!(content_range.range(), None);
        assert_eq!(content_range.size(), Some(100));
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Invalid Content-Range value")]
    fn bytes_content_no_range_and_no_size() {
        BytesContentRange::from_str("*/*").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid Content-Range value")]
    fn bytes_content_invalid_range() {
        BytesContentRange::from_str("a-b/*").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid Content-Range value")]
    fn bytes_content_invalid_size() {
        BytesContentRange::from_str("*/abc").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid Content-Range value")]
    fn bytes_content_invalid_range_end() {
        BytesContentRange::from_str("5-4/*").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid Content-Range value")]
    fn bytes_content_range_overflow_size() {
        BytesContentRange::from_str("1-4/3").unwrap();
    }

    #[test]
    fn bytes_content_apply_format_range_and_size() {
        let content_range = BytesContentRange::new().with_range(1, 5).with_size(100);
        assert_eq!(content_range.to_string(), "1-5/100");
    }

    #[test]
    fn bytes_content_apply_format_range_and_no_size() {
        let content_range = BytesContentRange::new().with_range(1, 5);
        assert_eq!(content_range.to_string(), "1-5/*");
    }

    #[test]
    fn bytes_content_apply_no_range_and_size() {
        let content_range = BytesContentRange::new().with_size(100);
        assert_eq!(content_range.to_string(), "*/100");
    }
}
