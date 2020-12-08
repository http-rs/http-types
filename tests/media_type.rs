use async_std::fs;
use async_std::io;
use http_types::{media_type, Body, Response};

#[async_std::test]
async fn guess_plain_text_media_type() -> io::Result<()> {
    let body = Body::from_file("tests/fixtures/index.html").await?;
    let mut res = Response::new(200);
    res.set_body(body);
    assert_eq!(res.content_type(), Some(media_type::HTML));
    Ok(())
}

#[async_std::test]
async fn guess_binary_media_type() -> http_types::Result<()> {
    let body = Body::from_file("tests/fixtures/nori.png").await?;
    let mut res = Response::new(200);
    res.set_body(body);
    assert_eq!(res.content_type(), Some(media_type::PNG));

    // Assert the file is correctly reset after we've peeked the bytes
    let left = fs::read("tests/fixtures/nori.png").await?;
    let right = res.body_bytes().await?;
    assert_eq!(left, right);
    Ok(())
}

#[async_std::test]
async fn guess_media_type_fallback() -> io::Result<()> {
    let body = Body::from_file("tests/fixtures/unknown.custom").await?;
    let mut res = Response::new(200);
    res.set_body(body);
    assert_eq!(res.content_type(), Some(media_type::BYTE_STREAM));
    Ok(())
}

#[async_std::test]
async fn parse_empty_files() -> http_types::Result<()> {
    let body = Body::from_file("tests/fixtures/empty.custom").await?;
    let mut res = Response::new(200);
    res.set_body(body);
    assert_eq!(res.content_type(), Some(media_type::BYTE_STREAM));
    Ok(())
}
