use http_types::{Error, ErrorKind, StatusCode, ensure, bail};
use std::io;

#[test]
fn can_be_boxed() {
    fn can_be_boxed() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let err = io::Error::new(io::ErrorKind::Other, "Oh no");
        Err(Error::from_io(err, StatusCode::NotFound))?;
        Ok(())
    }
    assert!(can_be_boxed().is_err());
}


#[test]
fn ensure() {
    fn inner() -> http_types::Result<()> {
        ensure!(1 == 1, "Oh yes");
        bail!("Oh no!");
    }
    let res = inner();
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.status(), StatusCode::InternalServerError);
    assert_eq!(err.kind(), ErrorKind::Other);
}