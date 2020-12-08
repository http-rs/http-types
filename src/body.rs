use futures_lite::{io, prelude::*, ready};
use serde::{de::DeserializeOwned, Serialize};

use std::fmt::{self, Debug};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::{media_type, MediaType};
use crate::{Status, StatusCode};

pin_project_lite::pin_project! {
    /// A streaming HTTP body.
    ///
    /// `Body` represents the HTTP body of both `Request` and `Response`. It's completely
    /// streaming, and implements `AsyncBufRead` to make reading from it both convenient and
    /// performant.
    ///
    /// Both `Request` and `Response` take `Body` by `Into<Body>`, which means that passing string
    /// literals, byte vectors, but also concrete `Body` instances are all valid. This makes it
    /// easy to create both quick HTTP requests, but also have fine grained control over how bodies
    /// are streamed out.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_types::{Body, Response, StatusCode};
    /// use async_std::io::Cursor;
    ///
    /// let mut req = Response::new(StatusCode::Ok);
    /// req.set_body("Hello Chashu");
    ///
    /// let mut req = Response::new(StatusCode::Ok);
    /// let cursor = Cursor::new("Hello Nori");
    /// let body = Body::from_reader(cursor, Some(10)); // set the body length
    /// req.set_body(body);
    /// ```
    ///
    /// # Length
    ///
    /// One of the details of `Body` to be aware of is the `length` parameter. The value of
    /// `length` is used by HTTP implementations to determine how to treat the stream. If a length
    /// is known ahead of time, it's _strongly_ recommended to pass it.
    ///
    /// Casting from `Vec<u8>`, `String`, or similar to `Body` will automatically set the value of
    /// `length`.
    ///
    /// # Content Encoding
    ///
    /// By default `Body` will come with a fallback MediaType type that is used by `Request` and
    /// `Response` if no other type has been set, and no other MediaType type can be inferred.
    ///
    /// It's _strongly_ recommended to always set a media_type type on both the `Request` and `Response`,
    /// and not rely on the fallback mechanisms. However, they're still there if you need them.
    pub struct Body {
        #[pin]
        reader: Box<dyn AsyncBufRead + Unpin + Send + Sync + 'static>,
        media_type: MediaType,
        length: Option<usize>,
        bytes_read: usize
    }
}

impl Body {
    /// Create a new empty `Body`.
    ///
    /// The body will have a length of `0`, and the MediaType type set to `application/octet-stream` if
    /// no other media_type type has been set or can be sniffed.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_types::{Body, Response, StatusCode};
    ///
    /// let mut req = Response::new(StatusCode::Ok);
    /// req.set_body(Body::empty());
    /// ```
    pub fn empty() -> Self {
        Self {
            reader: Box::new(io::empty()),
            media_type: media_type::BYTE_STREAM,
            length: Some(0),
            bytes_read: 0,
        }
    }

    /// Create a `Body` from a reader with an optional length.
    ///
    /// The MediaType type is set to `application/octet-stream` if no other media_type type has been set or can
    /// be sniffed. If a `Body` has no length, HTTP implementations will often switch over to
    /// framed messages such as [Chunked
    /// Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Transfer-Encoding).
    ///
    /// # Examples
    ///
    /// ```
    /// use http_types::{Body, Response, StatusCode};
    /// use async_std::io::Cursor;
    ///
    /// let mut req = Response::new(StatusCode::Ok);
    ///
    /// let cursor = Cursor::new("Hello Nori");
    /// let len = 10;
    /// req.set_body(Body::from_reader(cursor, Some(len)));
    /// ```
    pub fn from_reader(
        reader: impl AsyncBufRead + Unpin + Send + Sync + 'static,
        len: Option<usize>,
    ) -> Self {
        Self {
            reader: Box::new(reader),
            media_type: media_type::BYTE_STREAM,
            length: len,
            bytes_read: 0,
        }
    }

    /// Get the inner reader from the `Body`
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io::prelude::*;
    /// use http_types::Body;
    /// use async_std::io::Cursor;
    ///
    /// let cursor = Cursor::new("Hello Nori");
    /// let body = Body::from_reader(cursor, None);
    /// let _ = body.into_reader();
    /// ```
    pub fn into_reader(self) -> Box<dyn AsyncBufRead + Unpin + Send + Sync + 'static> {
        self.reader
    }

    /// Create a `Body` from a Vec of bytes.
    ///
    /// The MediaType type is set to `application/octet-stream` if no other media_type type has been set or can
    /// be sniffed. If a `Body` has no length, HTTP implementations will often switch over to
    /// framed messages such as [Chunked
    /// Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Transfer-Encoding).
    ///
    /// # Examples
    ///
    /// ```
    /// use http_types::{Body, Response, StatusCode};
    /// use async_std::io::Cursor;
    ///
    /// let mut req = Response::new(StatusCode::Ok);
    ///
    /// let input = vec![1, 2, 3];
    /// req.set_body(Body::from_bytes(input));
    /// ```
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            media_type: media_type::BYTE_STREAM,
            length: Some(bytes.len()),
            reader: Box::new(io::Cursor::new(bytes)),
            bytes_read: 0,
        }
    }

    /// Parse the body into a `Vec<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> http_types::Result<()> { async_std::task::block_on(async {
    /// use http_types::Body;
    ///
    /// let bytes = vec![1, 2, 3];
    /// let body = Body::from_bytes(bytes);
    ///
    /// let bytes: Vec<u8> = body.into_bytes().await?;
    /// assert_eq!(bytes, vec![1, 2, 3]);
    /// # Ok(()) }) }
    /// ```
    pub async fn into_bytes(mut self) -> crate::Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(1024);
        self.read_to_end(&mut buf)
            .await
            .status(StatusCode::UnprocessableEntity)?;
        Ok(buf)
    }

    /// Create a `Body` from a String
    ///
    /// The MediaType type is set to `text/plain` if no other media_type type has been set or can
    /// be sniffed. If a `Body` has no length, HTTP implementations will often switch over to
    /// framed messages such as [Chunked
    /// Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Transfer-Encoding).
    ///
    /// # Examples
    ///
    /// ```
    /// use http_types::{Body, Response, StatusCode};
    /// use async_std::io::Cursor;
    ///
    /// let mut req = Response::new(StatusCode::Ok);
    ///
    /// let input = String::from("hello Nori!");
    /// req.set_body(Body::from_string(input));
    /// ```
    pub fn from_string(s: String) -> Self {
        Self {
            media_type: media_type::PLAIN,
            length: Some(s.len()),
            reader: Box::new(io::Cursor::new(s.into_bytes())),
            bytes_read: 0,
        }
    }

    /// Read the body as a string
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> http_types::Result<()> { async_std::task::block_on(async {
    /// use http_types::Body;
    /// use async_std::io::Cursor;
    ///
    /// let cursor = Cursor::new("Hello Nori");
    /// let body = Body::from_reader(cursor, None);
    /// assert_eq!(&body.into_string().await.unwrap(), "Hello Nori");
    /// # Ok(()) }) }
    /// ```
    pub async fn into_string(mut self) -> crate::Result<String> {
        let mut result = String::with_capacity(self.len().unwrap_or(0));
        self.read_to_string(&mut result)
            .await
            .status(StatusCode::UnprocessableEntity)?;
        Ok(result)
    }

    /// Creates a `Body` from a type, serializing it as JSON.
    ///
    /// # MediaType
    ///
    /// The encoding is set to `application/json`.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_types::{Body, convert::json};
    ///
    /// let body = Body::from_json(&json!({ "name": "Chashu" }));
    /// # drop(body);
    /// ```
    pub fn from_json(json: &impl Serialize) -> crate::Result<Self> {
        let bytes = serde_json::to_vec(&json)?;
        let body = Self {
            length: Some(bytes.len()),
            reader: Box::new(io::Cursor::new(bytes)),
            media_type: media_type::JSON,
            bytes_read: 0,
        };
        Ok(body)
    }

    /// Parse the body as JSON, serializing it to a struct.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> http_types::Result<()> { async_std::task::block_on(async {
    /// use http_types::Body;
    /// use http_types::convert::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Cat { name: String }
    ///
    /// let cat = Cat { name: String::from("chashu") };
    /// let body = Body::from_json(&cat)?;
    ///
    /// let cat: Cat = body.into_json().await?;
    /// assert_eq!(&cat.name, "chashu");
    /// # Ok(()) }) }
    /// ```
    pub async fn into_json<T: DeserializeOwned>(mut self) -> crate::Result<T> {
        let mut buf = Vec::with_capacity(1024);
        self.read_to_end(&mut buf).await?;
        Ok(serde_json::from_slice(&buf).status(StatusCode::UnprocessableEntity)?)
    }

    /// Creates a `Body` from a type, serializing it using form encoding.
    ///
    /// # MediaType
    ///
    /// The encoding is set to `application/x-www-form-urlencoded`.
    ///
    /// # Errors
    ///
    /// An error will be returned if the encoding failed.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> http_types::Result<()> { async_std::task::block_on(async {
    /// use http_types::Body;
    /// use http_types::convert::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Cat { name: String }
    ///
    /// let cat = Cat { name: String::from("chashu") };
    /// let body = Body::from_form(&cat)?;
    ///
    /// let cat: Cat = body.into_form().await?;
    /// assert_eq!(&cat.name, "chashu");
    /// # Ok(()) }) }
    /// ```
    pub fn from_form(form: &impl Serialize) -> crate::Result<Self> {
        let query = serde_urlencoded::to_string(form)?;
        let bytes = query.into_bytes();

        let body = Self {
            length: Some(bytes.len()),
            reader: Box::new(io::Cursor::new(bytes)),
            media_type: media_type::FORM,
            bytes_read: 0,
        };
        Ok(body)
    }

    /// Parse the body from form encoding into a type.
    ///
    /// # Errors
    ///
    /// An error is returned if the underlying IO stream errors, or if the body
    /// could not be deserialized into the type.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> http_types::Result<()> { async_std::task::block_on(async {
    /// use http_types::Body;
    /// use http_types::convert::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Cat { name: String }
    ///
    /// let cat = Cat { name: String::from("chashu") };
    /// let body = Body::from_form(&cat)?;
    ///
    /// let cat: Cat = body.into_form().await?;
    /// assert_eq!(&cat.name, "chashu");
    /// # Ok(()) }) }
    /// ```
    pub async fn into_form<T: DeserializeOwned>(self) -> crate::Result<T> {
        let s = self.into_string().await?;
        Ok(serde_urlencoded::from_str(&s).status(StatusCode::UnprocessableEntity)?)
    }

    /// Create a `Body` from a file.
    ///
    /// The MediaType type set to `application/octet-stream` if no other media_type type has
    /// been set or can be sniffed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> http_types::Result<()> { async_std::task::block_on(async {
    /// use http_types::{Body, Response, StatusCode};
    ///
    /// let mut res = Response::new(StatusCode::Ok);
    /// res.set_body(Body::from_file("/path/to/file").await?);
    /// # Ok(()) }) }
    /// ```
    #[cfg(all(feature = "fs", not(target_os = "unknown")))]
    pub async fn from_file<P>(path: P) -> io::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let path = path.as_ref();
        let mut file = async_std::fs::File::open(path).await?;
        let len = file.metadata().await?.len();

        // Look at magic bytes first, look at extension second, fall back to
        // octet stream.
        let media_type = peek_media_type(&mut file)
            .await?
            .or_else(|| guess_ext(path))
            .unwrap_or(media_type::BYTE_STREAM);

        Ok(Self {
            media_type,
            length: Some(len as usize),
            reader: Box::new(io::BufReader::new(file)),
            bytes_read: 0,
        })
    }

    /// Get the length of the body in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_types::Body;
    /// use async_std::io::Cursor;
    ///
    /// let cursor = Cursor::new("Hello Nori");
    /// let len = 10;
    /// let body = Body::from_reader(cursor, Some(len));
    /// assert_eq!(body.len(), Some(10));
    /// ```
    pub fn len(&self) -> Option<usize> {
        self.length
    }

    /// Returns `true` if the body has a length of zero, and `false` otherwise.
    pub fn is_empty(&self) -> Option<bool> {
        self.length.map(|length| length == 0)
    }

    /// Returns the media_type type of this Body.
    pub fn media_type(&self) -> &MediaType {
        &self.media_type
    }

    /// Sets the media_type type of this Body.
    pub fn set_media_type(&mut self, media_type: impl Into<MediaType>) {
        self.media_type = media_type.into();
    }
}

impl Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Body")
            .field("reader", &"<hidden>")
            .field("length", &self.length)
            .field("bytes_read", &self.bytes_read)
            .finish()
    }
}

impl From<serde_json::Value> for Body {
    fn from(json_value: serde_json::Value) -> Self {
        Self::from_json(&json_value).unwrap()
    }
}

impl From<String> for Body {
    fn from(s: String) -> Self {
        Self::from_string(s)
    }
}

impl<'a> From<&'a str> for Body {
    fn from(s: &'a str) -> Self {
        Self::from_string(s.to_owned())
    }
}

impl From<Vec<u8>> for Body {
    fn from(b: Vec<u8>) -> Self {
        Self::from_bytes(b)
    }
}

impl<'a> From<&'a [u8]> for Body {
    fn from(b: &'a [u8]) -> Self {
        Self::from_bytes(b.to_owned())
    }
}

impl AsyncRead for Body {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut buf = match self.length {
            None => buf,
            Some(length) if length == self.bytes_read => return Poll::Ready(Ok(0)),
            Some(length) => {
                let max_len = (length - self.bytes_read).min(buf.len());
                &mut buf[0..max_len]
            }
        };

        let bytes = ready!(Pin::new(&mut self.reader).poll_read(cx, &mut buf))?;
        self.bytes_read += bytes;
        Poll::Ready(Ok(bytes))
    }
}

impl AsyncBufRead for Body {
    #[allow(missing_doc_code_examples)]
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&'_ [u8]>> {
        self.project().reader.poll_fill_buf(cx)
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        Pin::new(&mut self.reader).consume(amt)
    }
}

/// Look at first few bytes of a file to determine the media_type type.
/// This is used for various binary formats such as images and videos.
#[cfg(all(feature = "fs", not(target_os = "unknown")))]
async fn peek_media_type(file: &mut async_std::fs::File) -> io::Result<Option<MediaType>> {
    // We need to read the first 300 bytes to correctly infer formats such as tar.
    let mut buf = [0_u8; 300];
    file.read(&mut buf).await?;
    let media_type = MediaType::sniff(&buf).ok();

    // Reset the file cursor back to the start.
    file.seek(io::SeekFrom::Start(0)).await?;
    Ok(media_type)
}

/// Look at the extension of a file to determine the media_type type.
/// This is useful for plain-text formats such as HTML and CSS.
#[cfg(all(feature = "fs", not(target_os = "unknown")))]
fn guess_ext(path: &std::path::Path) -> Option<MediaType> {
    let ext = path.extension().map(|p| p.to_str()).flatten();
    ext.and_then(MediaType::from_extension)
}

#[cfg(test)]
mod test {
    use super::*;
    use async_std::io::Cursor;
    use serde::Deserialize;

    #[async_std::test]
    async fn json_status() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            inner: String,
        }
        let body = Body::empty();
        let res = body.into_json::<Foo>().await;
        assert_eq!(res.unwrap_err().status(), 422);
    }

    #[async_std::test]
    async fn form_status() {
        #[derive(Debug, Deserialize)]
        struct Foo {
            inner: String,
        }
        let body = Body::empty();
        let res = body.into_form::<Foo>().await;
        assert_eq!(res.unwrap_err().status(), 422);
    }

    async fn read_with_buffers_of_size<R>(reader: &mut R, size: usize) -> crate::Result<String>
    where
        R: AsyncRead + Unpin,
    {
        let mut return_buffer = vec![];
        loop {
            let mut buf = vec![0; size];
            match reader.read(&mut buf).await? {
                0 => break Ok(String::from_utf8(return_buffer)?),
                bytes_read => return_buffer.extend_from_slice(&buf[..bytes_read]),
            }
        }
    }

    #[async_std::test]
    async fn attempting_to_read_past_length() -> crate::Result<()> {
        for buf_len in 1..13 {
            let mut body = Body::from_reader(Cursor::new("hello world"), Some(5));
            assert_eq!(
                read_with_buffers_of_size(&mut body, buf_len).await?,
                "hello"
            );
            assert_eq!(body.bytes_read, 5);
        }

        Ok(())
    }

    #[async_std::test]
    async fn attempting_to_read_when_length_is_greater_than_content() -> crate::Result<()> {
        for buf_len in 1..13 {
            let mut body = Body::from_reader(Cursor::new("hello world"), Some(15));
            assert_eq!(
                read_with_buffers_of_size(&mut body, buf_len).await?,
                "hello world"
            );
            assert_eq!(body.bytes_read, 11);
        }

        Ok(())
    }

    #[async_std::test]
    async fn attempting_to_read_when_length_is_exactly_right() -> crate::Result<()> {
        for buf_len in 1..13 {
            let mut body = Body::from_reader(Cursor::new("hello world"), Some(11));
            assert_eq!(
                read_with_buffers_of_size(&mut body, buf_len).await?,
                "hello world"
            );
            assert_eq!(body.bytes_read, 11);
        }

        Ok(())
    }

    #[async_std::test]
    async fn reading_in_various_buffer_lengths_when_there_is_no_length() -> crate::Result<()> {
        for buf_len in 1..13 {
            let mut body = Body::from_reader(Cursor::new("hello world"), None);
            assert_eq!(
                read_with_buffers_of_size(&mut body, buf_len).await?,
                "hello world"
            );
            assert_eq!(body.bytes_read, 11);
        }

        Ok(())
    }
}
