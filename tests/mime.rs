use http_types::{Response, Body, mime};
use async_std::io;

#[async_std::test]
async fn guess_plain_text_mime() -> io::Result<()> {
    let body = Body::from_file("tests/fixtures/index.html").await?;
    let mut res = Response::new(200);
    res.set_body(body);
    assert_eq!(res.content_type(), Some(mime::HTML));
    Ok(())
}

#[async_std::test]
async fn guess_binary_mime() -> io::Result<()> {
    let body = Body::from_file("tests/fixtures/nori.png").await?;
    let mut res = Response::new(200);
    res.set_body(body);
    assert_eq!(res.content_type(), Some(mime::PNG));
    Ok(())
}
