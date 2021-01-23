use futures_lite::{future, AsyncReadExt};
use http_types::{Body, Method, Request, Response, StatusCode};

#[test]
fn test_req_res_set_body() {
    let mut req = Request::new(Method::Get, "http://example.com/").unwrap();
    req.set_body(Body::empty());
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(req);
    let body = future::block_on(async move {
        let mut body = Vec::new();
        res.read_to_end(&mut body).await.unwrap();
        body
    });
    assert!(body.is_empty());
}

#[test]
fn test_req_res_take_replace_body() {
    let mut req = Request::new(Method::Get, "http://example.com/").unwrap();
    req.take_body();
    let mut res = Response::new(StatusCode::Ok);
    res.replace_body(req);
    let body = future::block_on(async move {
        let mut body = Vec::new();
        res.read_to_end(&mut body).await.unwrap();
        body
    });
    assert!(body.is_empty());
}
