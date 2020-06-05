use http_types::{bail, ensure, ensure_eq, Error, Response, StatusCode};
use std::io;

#[test]
fn can_be_boxed() {
    fn can_be_boxed() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let err = io::Error::new(io::ErrorKind::Other, "Oh no");
        Err(Error::new(StatusCode::NotFound, err).into())
    }
    assert!(can_be_boxed().is_err());
}

#[test]
fn internal_server_error_by_default() {
    fn run() -> http_types::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "Oh no"))?;
        Ok(())
    }
    let err = run().unwrap_err();
    assert_eq!(err.status(), 500);
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
}

#[test]
fn ensure_eq() {
    fn inner() -> http_types::Result<()> {
        ensure_eq!(1, 1, "Oh yes");
        bail!("Oh no!");
    }
    let res = inner();
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.status(), StatusCode::InternalServerError);
}

#[test]
fn result_ext() {
    use http_types::Status;
    fn run() -> http_types::Result<()> {
        let err = io::Error::new(io::ErrorKind::Other, "Oh no");
        Err(err).status(StatusCode::NotFound)?;
        Ok(())
    }
    let res = run();
    assert!(res.is_err());

    let err = res.unwrap_err();
    assert_eq!(err.status(), StatusCode::NotFound);
}

#[test]
fn option_ext() {
    use http_types::Status;
    fn run() -> http_types::Result<()> {
        None.status(StatusCode::NotFound)
    }
    let res = run();
    assert!(res.is_err());

    let err = res.unwrap_err();
    assert_eq!(err.status(), StatusCode::NotFound);
}

#[test]
fn to_response() {
    let msg = "This is an error";

    let error = Error::from_str(StatusCode::NotFound, msg);
    let mut res: Response = error.into();

    assert!(res.error().is_some());
    // Ensure we did not consume the error
    assert!(res.error().is_some());

    assert_eq!(res.error().unwrap().status(), StatusCode::NotFound);
    assert_eq!(res.error().unwrap().to_string(), msg);

    res.take_error();
    assert!(res.error().is_none());
}
