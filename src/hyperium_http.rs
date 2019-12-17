// This is the compat file for the "hyperium/http" crate.

use crate::headers::{HeaderName, HeaderValue};
use crate::{Body, Headers, Method, Request, Response, StatusCode, Url};
use std::convert::TryFrom;
use std::str::FromStr;

impl From<http::Method> for Method {
    fn from(method: http::Method) -> Self {
        Method::from_str(method.as_str()).unwrap()
    }
}

impl From<&Method> for http::Method {
    fn from(method: &Method) -> Self {
        http::Method::from_str(&format!("{}", method)).unwrap()
    }
}

impl From<http::StatusCode> for StatusCode {
    fn from(status: http::StatusCode) -> Self {
        StatusCode::try_from(status.as_u16()).unwrap()
    }
}

impl From<StatusCode> for http::StatusCode {
    fn from(status: StatusCode) -> Self {
        http::StatusCode::from_u16(status.into()).unwrap()
    }
}

fn hyperium_headers_to_headers(hyperium_headers: http::HeaderMap, headers: &mut Headers) {
    for (name, value) in hyperium_headers {
        let value = value.as_bytes().to_owned();
        let value = unsafe { HeaderValue::from_ascii_unchecked(value) };
        let name = name.unwrap().as_str().as_bytes().to_owned();
        let name = unsafe { HeaderName::from_ascii_unchecked(name) };
        headers.insert(name, value).unwrap();
    }
}

fn headers_to_hyperium_headers(headers: &mut Headers, hyperium_headers: &mut http::HeaderMap) {
    for (name, values) in headers {
        let name = format!("{}", name).into_bytes();
        let name = http::header::HeaderName::from_bytes(&name).unwrap();

        for value in values {
            let value = format!("{}", value).into_bytes();
            let value = http::header::HeaderValue::from_bytes(&value).unwrap();
            hyperium_headers.append(&name, value);
        }
    }
}

// Neither type is defined in this lib, so we can't do From/Into impls
fn from_uri_to_url(uri: http::Uri) -> Result<Url, crate::url::ParseError> {
    format!("{}", uri).parse()
}

// Neither type is defined in this lib, so we can't do From/Into impls
fn from_url_to_uri(url: &Url) -> http::Uri {
    http::Uri::try_from(&format!("{}", url)).unwrap()
}

// TODO: move HTTP version over if it exists.
impl TryFrom<http::Request<Body>> for Request {
    type Error = crate::url::ParseError;

    fn try_from(req: http::Request<Body>) -> Result<Self, Self::Error> {
        let (parts, body) = req.into_parts();
        let method = parts.method.into();
        let url = from_uri_to_url(parts.uri)?;
        let mut req = Request::new(method, url);
        req.set_body(body);
        hyperium_headers_to_headers(parts.headers, req.as_mut());
        Ok(req)
    }
}

// TODO: move HTTP version over if it exists.
impl From<Request> for http::Request<Body> {
    fn from(mut req: Request) -> Self {
        let method: http::Method = req.method().into();
        let mut builder = http::request::Builder::new()
            .method(method)
            .uri(from_url_to_uri(req.url()))
            .version(http::version::Version::default());
        headers_to_hyperium_headers(req.as_mut(), builder.headers_mut().unwrap());
        builder.body(req.into()).unwrap()
    }
}

// TODO: move HTTP version over if it exists.
impl From<http::Response<Body>> for Response {
    fn from(res: http::Response<Body>) -> Self {
        let (parts, body) = res.into_parts();
        let status = parts.status.into();
        let mut res = Response::new(status);
        res.set_body(body);
        hyperium_headers_to_headers(parts.headers, res.as_mut());
        res
    }
}

// TODO: move HTTP version over if it exists.
impl From<Response> for http::Response<Body> {
    fn from(mut res: Response) -> Self {
        let status: u16 = res.status().into();
        let mut builder = http::response::Builder::new()
            .status(status)
            .version(http::version::Version::default());
        headers_to_hyperium_headers(res.as_mut(), builder.headers_mut().unwrap());
        builder.body(res.into()).unwrap()
    }
}
