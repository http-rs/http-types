use http_types::{Error, StatusCode};
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
