use std::time::Duration;

/// An individual [`ServerTiming`] entry.
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
pub struct TimingEntry {
    name: String,
    dur: Option<Duration>,
    desc: Option<String>,
}

impl TimingEntry {
    /// Create a new instance of `TimingEntry`.
    pub fn new(name: String, dur: Option<Duration>, desc: Option<String>) -> Self {
        Self { name, dur, desc }
    }

    /// The timing name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Set the timing name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// The timing duration.
    pub fn duration(&self) -> Option<Duration> {
        self.dur
    }

    /// Set the timing name.
    pub fn set_duration(&mut self, dur: Option<Duration>) {
        self.dur = dur;
    }

    /// The timing description.
    pub fn description(&self) -> Option<&String> {
        self.desc.as_ref()
    }

    /// Set the timing description.
    pub fn set_description(&mut self, desc: Option<String>) {
        self.desc = desc;
    }
}

/// Communicate one or more metrics and descriptions for the given request-response cycle.
///
/// This is an implementation of the W3C [Server
/// Timing](https://w3c.github.io/server-timing/#the-server-timing-header-field)
/// header spec. Read more on
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Server-Timing).
#[derive(Debug)]
pub struct ServerTiming {
    timings: Vec<TimingEntry>,
}

mod test {
    const CASE1: &str =
        "Server-Timing: metric1; dur=1.1; desc=document, metric1; dur=1.2; desc=document";
}
