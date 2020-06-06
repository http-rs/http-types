//! Multipart/form-data types.
//!
//! # Examples
//!
//! Request:
//! ```
//! use http_types::multipart::{Multipart, Entry};
//!
//! let mut req = Request::new(Method::Get, "http://example.website");
//!
//! let mut multi = Multipart::new();
//! multi.push(Entry::new("description", "hello world"));
//!
//! let mut entry = Entry::from_file("my_file", Body::from_file("./cats.jpeg").await?);
//! entry.set_file_name("cats.jpeg");
//! multi.push("myFile", Body::from_file("./cats.jpeg").await?);
//!
//! req.set_body(multi);
//! ```
//!
//! Response:
//!
//! ```
//! use http_types::multipart::{Multipart, Entry};
//! let mut res = Response::new(200); // get this from somewhere
//!
//! let mut entries = res.body_multipart();
//! while let Some(entry) = entries.await {
//!     println!("name: {}", entry.name());
//!     println!("data: {}", entry.into_string()?);
//! }
//! ```

use std::io::{Cursor, Read};
use std::task::Context;
use std::task::Poll;
use std::{fmt::Debug, pin::Pin, str::FromStr};

use futures_core::stream::Stream;
use futures_lite::{io, prelude::*};
use futures_util::stream::TryStreamExt;
use multipart::server::Multipart as Parser;

use crate::mime;
use crate::{format_err, Body, Mime, Status};
pub use entry::Entry;

mod entry;

/// A multipart response body.
pub struct Multipart {
    entries: Vec<Entry>,
    body: Option<Parser<Cursor<String>>>,
}

impl Debug for Multipart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Multipart").finish()
    }
}

impl Multipart {
    /// Create a new instance of `Multipart`.
    pub fn new() -> Self {
        Self {
            entries: vec![],
            body: None,
        }
    }

    /// Parse a `Body` stream as a `Multipart` instance.
    pub async fn from_req(req: &mut crate::Request) -> crate::Result<Self> {
        let boundary = req
            .content_type()
            .map(|ct| ct.param("boundary").cloned())
            .flatten();

        let boundary = match boundary {
            Some(boundary) => boundary.as_str().to_owned(),
            None => {
                let mut err =
                    format_err!("Invalid `Content-Type` header. Expected a `boundary` param");
                err.set_status(400);
                return Err(err);
            }
        };

        // Not ideal, but done for now so we can avoid implementing all of Multipart ourselves for the time being.
        let body = req.take_body().into_string().await?;

        let multipart = Parser::with_body(Cursor::new(body), boundary);
        Ok(Self {
            entries: vec![],
            body: Some(multipart),
        })
    }

    /// Add a new entry to the `Multipart` instance.
    pub fn push<E>(&mut self, entry: E)
    where
        E: Into<Entry>,
    {
        self.entries.push(entry.into());
        // if let Some(entries) = self.entries.as_mut() {
        //     entries.push(entry.into());
        // } else {
        //     self.entries = Some(vec![entry.into()]);
        // }
    }
}

impl Stream for Multipart {
    type Item = crate::Result<Entry>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<Self::Item>> {
        let body = match self.body.as_mut() {
            None => return Poll::Ready(None),
            Some(body) => body,
        };

        match body.read_entry() {
            Ok(Some(mut field)) => {
                let mut body = vec![];
                field.data.read_to_end(&mut body).status(400)?;

                let mut entry = Entry::new(field.headers.name, body);
                entry.set_file_name(field.headers.filename);
                let mime = field
                    .headers
                    .content_type
                    .map(|ct| Mime::from_str(&ct.to_string()))
                    .transpose()?;
                if let Some(mime) = mime {
                    entry.set_mime(mime);
                } else {
                    // https://tools.ietf.org/html/rfc7578#section-4.4
                    entry.set_mime(mime::PLAIN);
                }

                Poll::Ready(Some(Ok(entry)))
            }
            Ok(None) => Poll::Ready(None),
            Err(e) => {
                let mut err = format_err!("Invalid multipart entry: {}", e);
                err.set_status(400);
                Poll::Ready(Some(Err(err)))
            }
        }
    }
}

// struct MultipartReader {
//     entry_iter: Box<dyn Iterator<Item = Entry>>,
// }

// impl From<Multipart> for MultipartReader {
//     fn from(multipart: Multipart) -> Self {
//         Self {
//             entry_iter: Box::new(multipart.entries.into_iter())
//         }
//     }
// }

// impl AsyncRead for MultipartReader {
//     #[allow(missing_doc_code_examples)]
//     fn poll_read(
//         mut self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//         buf: &mut [u8],
//     ) -> Poll<io::Result<usize>> {
//         if let Some(entry) = self.entry_iter.next() {
//             Pin::new(&mut entry).poll_read(cx, buf)
//         } else {
//             Poll::Ready()
//         }
//     }
// }

// impl AsyncBufRead for MultipartReader {
//     #[allow(missing_doc_code_examples)]
//     fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&'_ [u8]>> {
//         let this = self.project();
//         this.reader.poll_fill_buf(cx)
//     }

//     fn consume(mut self: Pin<&mut Self>, amt: usize) {
//         Pin::new(&mut self.reader).consume(amt)
//     }
// }

// We need AsRef<[u8]> on BufReader for TryStreamExt (into_async_read) so... wrap and patch it in ourselves, for now.
#[doc(hidden)]
#[derive(Debug)]
pub struct BufReader<R: AsyncRead> {
    inner: io::BufReader<R>,
}

#[doc(hidden)]
impl<R: AsyncRead> BufReader<R> {
    #[allow(missing_doc_code_examples)]
    #[doc(hidden)]
    pub fn new(inner: R) -> Self {
        Self {
            inner: io::BufReader::new(inner),
        }
    }
}

#[doc(hidden)]
impl<R: AsyncRead> AsRef<[u8]> for BufReader<R> {
    #[allow(missing_doc_code_examples)]
    #[doc(hidden)]
    fn as_ref(&self) -> &[u8] {
        self.inner.buffer()
    }
}

impl From<Multipart> for Body {
    fn from(multipart: Multipart) -> Self {
        let stream = multipart.map(|maybe_entry| {
            maybe_entry
                .map(BufReader::new)
                .map_err(|err| {
                    std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
                })
        });
        let mut body = Body::from_reader(io::BufReader::new(stream.into_async_read()), None);
        body.set_mime(mime::MULTIPART_FORM);
        body
    }
}
