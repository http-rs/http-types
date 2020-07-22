use std::str::FromStr;
use std::time::Duration;

use crate::headers::HeaderValue;
use crate::Status;
use crate::{ensure, format_err};

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
    pub fn description(&self) -> Option<&str> {
        self.desc.as_ref().map(|s| s.as_str())
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

impl FromStr for Entry {
    type Err = crate::Error;
    // Create an entry from a string. Parsing rules in ABNF are:
    //
    // ```
    // Server-Timing             = #server-timing-metric
    // server-timing-metric      = metric-name *( OWS ";" OWS server-timing-param )
    // metric-name               = token
    // server-timing-param       = server-timing-param-name OWS "=" OWS server-timing-param-value
    // server-timing-param-name  = token
    // server-timing-param-value = token / quoted-string
    // ```
    //
    // Source: https://w3c.github.io/server-timing/#the-server-timing-header-field
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(';');

        // Get the name. This is non-optional.
        let name = parts
            .next()
            .ok_or_else(|| format_err!("Server timing headers must include a name"))?
            .trim_end();

        // We must extract these values from the k-v pairs that follow.
        let mut dur = None;
        let mut desc = None;

        for mut part in parts {
            ensure!(
                !part.is_empty(),
                "Server timing params cannot end with a trailing `;`"
            );

            part = part.trim_start();

            let mut params = part.split('=');
            let name = params
                .next()
                .ok_or_else(|| format_err!("Server timing params must have a name"))?
                .trim_end();
            let mut value = params
                .next()
                .ok_or_else(|| format_err!("Server timing params must have a value"))?
                .trim_start();

            match name {
                "dur" => {
                    let millis: f64 = value.parse().status(400).map_err(|_| {
                        format_err!("Server timing duration params must be a valid double-precision floating-point number.")
                    })?;
                    dur = Some(Duration::from_secs_f64(millis / 1000.0));
                }
                "desc" => {
                    // Ensure quotes line up, and strip them from the resulting output
                    if value.starts_with('"') {
                        value = &value[1..value.len()];
                        ensure!(
                            value.ends_with('"'),
                            "Server timing description params must use matching quotes"
                        );
                        value = &value[0..value.len() - 1];
                    } else {
                        ensure!(
                            !value.ends_with('"'),
                            "Server timing description params must use matching quotes"
                        );
                    }
                    desc = Some(value.to_string());
                }
                _ => continue,
            }
        }

        Ok(Entry {
            name: name.to_string(),
            dur,
            desc,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode() -> crate::Result<()> {
        let name = String::from("Server");
        let dur = Duration::from_secs(1);
        let desc = String::from("A server timing");

        let val: HeaderValue = Entry::new(name.clone(), None, None)?.into();
        assert_eq!(val, "Server");

        let val: HeaderValue = Entry::new(name.clone(), Some(dur), None)?.into();
        assert_eq!(val, "Server; dur=1000");

        let val: HeaderValue = Entry::new(name.clone(), None, Some(desc.clone()))?.into();
        assert_eq!(val, r#"Server; desc="A server timing""#);

        let val: HeaderValue = Entry::new(name.clone(), Some(dur), Some(desc.clone()))?.into();
        assert_eq!(val, r#"Server; dur=1000; desc="A server timing""#);
        Ok(())
    }

    #[test]
    fn decode() -> crate::Result<()> {
        // Metric name only.
        assert_entry("Server", "Server", None, None)?;
        assert_entry("Server ", "Server", None, None)?;
        assert_entry_err(
            "Server ;",
            "Server timing params cannot end with a trailing `;`",
        );
        assert_entry_err(
            "Server; ",
            "Server timing params cannot end with a trailing `;`",
        );

        // Metric name + param
        assert_entry("Server; dur=1000", "Server", Some(1000), None)?;
        assert_entry("Server; dur =1000", "Server", Some(1000), None)?;
        assert_entry("Server; dur= 1000", "Server", Some(1000), None)?;
        assert_entry("Server; dur = 1000", "Server", Some(1000), None)?;
        assert_entry_err(
            "Server; dur=1000;",
            "Server timing params cannot end with a trailing `;`",
        );

        // Metric name + desc
        assert_entry(r#"DB; desc="a db""#, "DB", None, Some("a db"))?;
        assert_entry(r#"DB; desc ="a db""#, "DB", None, Some("a db"))?;
        assert_entry(r#"DB; desc= "a db""#, "DB", None, Some("a db"))?;
        assert_entry(r#"DB; desc = "a db""#, "DB", None, Some("a db"))?;
        assert_entry(r#"DB; desc=a_db"#, "DB", None, Some("a_db"))?;
        assert_entry_err(
            r#"DB; desc="db"#,
            "Server timing description params must use matching quotes",
        );
        assert_entry_err(
            "Server; desc=a_db;",
            "Server timing params cannot end with a trailing `;`",
        );

        // Metric name + dur + desc
        assert_entry(
            r#"Server; dur=1000; desc="a server""#,
            "Server",
            Some(1000),
            Some("a server"),
        )?;
        assert_entry_err(
            r#"Server; dur=1000; desc="a server";"#,
            "Server timing params cannot end with a trailing `;`",
        );
        Ok(())
    }

    fn assert_entry_err(s: &str, msg: &str) {
        let err = Entry::from_str(s).unwrap_err();
        assert_eq!(format!("{}", err), msg);
    }

    /// Assert an entry and all of its fields.
    fn assert_entry(s: &str, n: &str, du: Option<u64>, de: Option<&str>) -> crate::Result<()> {
        let e = Entry::from_str(s)?;
        assert_eq!(e.name(), n);
        assert_eq!(e.duration(), du.map(|du| Duration::from_millis(du)));
        assert_eq!(e.description(), de);
        Ok(())
    }
}
