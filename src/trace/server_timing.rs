/// Communicate one or more metrics and descriptions for the given request-response cycle.
///
/// This is an implementation of the W3C [Server
/// Timing](https://w3c.github.io/server-timing/#the-server-timing-header-field)
/// header spec. Read more on
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Server-Timing)
pub struct ServerTiming;

// Four different cases are valid:
//
// 1. metric name only       cache
// 2. metric + metric        cache;dur=2.4
// 3. metric + description   cache;desc="Cache Read"
// 4. metric + value + desc  cache;desc="Cache Read";dur=23.2
//
// Multiple different entries per line are supported; separated with a `,`.

mod test {
    const CASE1: &str =
        "Server-Timing: metric1; dur=1.1; desc=document, metric1; dur=1.2; desc=document";
}
