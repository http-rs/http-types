use http_types::{url::Url, Method};
use serde_crate::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "serde_crate")]
struct Params {
    msg: String,
}

#[derive(Deserialize)]
#[serde(crate = "serde_crate")]
struct OptionalParams {
    _msg: Option<String>,
    _time: Option<u64>,
}

#[test]
fn successfully_deserialize_query() {
    let req = http_types::Request::new(
        Method::Get,
        Url::parse("http://example.com/?msg=Hello").unwrap(),
    );

    let params = req.query::<Params>();
    assert!(params.is_ok());
    assert_eq!(params.unwrap().msg, "Hello");
}

#[test]
fn unsuccessfully_deserialize_query() {
    let req = http_types::Request::new(Method::Get, Url::parse("http://example.com/").unwrap());

    let params = req.query::<Params>();
    assert!(params.is_err());
    assert_eq!(
        params.err().unwrap().to_string(),
        "Query string deserialization error: missing field `msg`"
    );
}

#[test]
fn malformatted_query() {
    let req = http_types::Request::new(
        Method::Get,
        Url::parse("http://example.com/?error=should_fail").unwrap(),
    );

    let params = req.query::<Params>();
    assert!(params.is_err());
    assert_eq!(
        params.err().unwrap().to_string(),
        "Query string deserialization error: missing field `msg`"
    );
}

#[test]
fn empty_query_string_for_struct_with_no_required_fields() {
    let req = http_types::Request::new(Method::Get, Url::parse("http://example.com").unwrap());

    let params = req.query::<OptionalParams>();
    assert!(params.is_ok());
}
