use http_types::{bail, ensure, ensure_eq, Error, ResponseError, StatusCode};
use std::io;

#[test]
fn can_be_boxed() {
    fn can_be_boxed() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let err = io::Error::new(io::ErrorKind::Other, "Oh no");
        Err(Error::IO(err).into())
    }
    assert!(can_be_boxed().is_err());
}

#[test]
fn internal_server_error_by_default() {
    fn run() -> http_types::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "Oh no").into())
    }
    let err = run().unwrap_err();
    assert_eq!(err.associated_status_code(), None);
}

#[test]
fn ensure() {
    fn inner() -> http_types::ResponseResult<()> {
        ensure!(true, "Oh yes");
        bail!("Oh no!");
    }
    let res = inner();
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.status(), None);
}

#[test]
fn ensure_eq() {
    fn inner() -> http_types::ResponseResult<()> {
        ensure_eq!(1, 1, "Oh yes");
        bail!("Oh no!");
    }
    let res = inner();
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.status(), None);
}

#[test]
fn result_ext() {
    use http_types::Status;
    fn run() -> http_types::ResponseResult<()> {
        let err = io::Error::new(io::ErrorKind::Other, "Oh no");
        Err(err).status(StatusCode::NotFound)?;
        Ok(())
    }
    let res = run();
    assert!(res.is_err());

    let err = res.unwrap_err();
    assert_eq!(err.status(), Some(StatusCode::NotFound));
}

#[test]
fn option_ext() {
    use http_types::Status;
    fn run() -> http_types::ResponseResult<()> {
        None.status(StatusCode::NotFound)
    }
    let res = run();
    assert!(res.is_err());

    let err = res.unwrap_err();
    assert_eq!(err.status(), Some(StatusCode::NotFound));
}

#[test]
fn anyhow_error_into_http_types_response_error() {
    let anyhow_error =
        anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, "irrelevant"));
    let http_types_error: ResponseError = anyhow_error.into();
    assert_eq!(http_types_error.status(), None);

    let anyhow_error =
        anyhow::Error::new(std::io::Error::new(std::io::ErrorKind::Other, "irrelevant"));
    let http_types_error: ResponseError =
        ResponseError::new_status(StatusCode::ImATeapot, anyhow_error);
    assert_eq!(http_types_error.status(), Some(StatusCode::ImATeapot));
}

#[test]
fn normal_error_into_http_types_response_error() {
    let http_types_error: ResponseError =
        std::io::Error::new(std::io::ErrorKind::Other, "irrelevant").into();
    assert_eq!(http_types_error.status(), None);

    let http_types_error = ResponseError::new_status(
        StatusCode::ImATeapot,
        std::io::Error::new(std::io::ErrorKind::Other, "irrelevant"),
    );
    assert_eq!(http_types_error.status(), Some(StatusCode::ImATeapot));
}

#[test]
fn u16_into_status_code_in_http_types_error() {
    let http_types_error = ResponseError::from_str_status(404, "Not Found");
    assert_eq!(http_types_error.status(), Some(StatusCode::NotFound));
}

#[test]
#[should_panic = "Could not convert into a valid `StatusCode`"]
fn fail_test_u16_into_status_code_in_http_types_error_new() {
    panic!(
        "{}",
        ResponseError::from_str_status(
            1000,
            io::Error::new(io::ErrorKind::Other, "Incorrect status code"),
        )
    )
}

#[test]
#[should_panic = "Could not convert into a valid `StatusCode`"]
fn fail_test_u16_into_status_code_in_http_types_error_from_str() {
    panic!(
        "{}",
        ResponseError::from_str_status(1000, "Incorrect status code",)
    )
}
