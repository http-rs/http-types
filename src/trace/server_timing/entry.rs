use std::time::Duration;

use crate::headers::HeaderValue;

/// An individual `ServerTiming` entry.
//
// # Implementation notes
//
// Four different cases are valid:
//
// 1. metric name only       cache
// 2. metric + value         cache;dur=2.4
// 3. metric + desc          cache;desc="Cache Read"
// 4. metric + value + desc  cache;desc="Cache Read";dur=23.2
//
// Multiple different entries per line are supported; separated with a `,`.
#[derive(Debug)]
pub struct Entry {
    name: String,
    dur: Option<Duration>,
    desc: Option<String>,
}

impl Entry {
    /// Create a new instance of `Entry`.
    ///
    /// # Errors
    ///
    /// An error will be returned if the string values are invalid ASCII.
    pub fn new(name: String, dur: Option<Duration>, desc: Option<String>) -> crate::Result<Self> {
        crate::ensure!(name.is_ascii(), "Name should be valid ASCII");
        if let Some(desc) = desc.as_ref() {
            crate::ensure!(desc.is_ascii(), "Description should be valid ASCII");
        };

        Ok(Self { name, dur, desc })
    }

    /// The timing name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The timing duration.
    pub fn duration(&self) -> Option<Duration> {
        self.dur
    }

    /// The timing description.
    pub fn description(&self) -> Option<&String> {
        self.desc.as_ref()
    }
}

impl From<Entry> for HeaderValue {
    fn from(entry: Entry) -> HeaderValue {
        let mut string = entry.name;

        // Format a `Duration` into the format that the spec expects.
        let f = |d: Duration| d.as_secs_f64() * 1000.0;

        match (entry.dur, entry.desc) {
            (Some(dur), Some(desc)) => {
                string.push_str(&format!("; dur={}; desc=\"{}\"", f(dur), desc))
            }
            (Some(dur), None) => string.push_str(&format!("; dur={}", f(dur))),
            (None, Some(desc)) => string.push_str(&format!("; desc=\"{}\"", desc)),
            (None, None) => {}
        };

        // SAFETY: we validate that the values are valid ASCII on creation.
        unsafe { HeaderValue::from_bytes_unchecked(string.into_bytes()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_header_value() -> crate::Result<()> {
        let name = String::from("Server");
        let dur = Duration::from_secs(1);
        let desc = String::from("A server timing");

        let val: HeaderValue = Entry::new(name.clone(), None, None)?.into();
        assert_eq!(val, "Server");

        let val: HeaderValue = Entry::new(name.clone(), Some(dur), None)?.into();
        assert_eq!(val, "Server; dur=1000");

        let val: HeaderValue = Entry::new(name.clone(), Some(dur), Some(desc.clone()))?.into();
        assert_eq!(val, r#"Server; dur=1000; desc="A server timing""#);
        Ok(())
    }
}
