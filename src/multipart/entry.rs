use crate::{Body, Mime};

use std::fmt::{self, Debug};
// use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_lite::{io, prelude::*};

pin_project_lite::pin_project! {
    /// A single multipart entry.
    ///
    /// Structurally Multipart entries are similar to `Body`.
    pub struct Entry {
        name: String,
        body: Body,
    }
}

impl Entry {
    /// Create a new `Entry`.
    pub fn new<S, B>(name: S, body: B) -> Self
    where
        S: AsRef<str>,
        B: Into<Body>,
    {
        Self {
            name: name.as_ref().to_owned(),
            body: body.into(),
        }
    }

    /// Create an empty `Entry`.
    pub fn empty<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        Self::new(name, Body::empty())
    }

    /// Create an `Entry` from a file.
    #[cfg(all(feature = "async_std", not(target_os = "unknown")))]
    pub async fn from_file<S, P>(name: S, path: P) -> crate::Result<Self>
    where
        S: AsRef<str>,
        P: AsRef<Path>,
    {
        let body = Body::from_file(path).await?;
        Ok(Self::new(name, body))
    }

    /// Get the entry name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Set the entry name.
    pub fn set_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.name = name.as_ref().to_owned();
    }

    /// Returns the mime type of this Body.
    pub fn mime(&self) -> &Mime {
        self.body.mime()
    }

    /// Sets the mime type of this Body.
    pub fn set_mime(&mut self, mime: Mime) {
        self.body.set_mime(mime)
    }

    /// Get the file name of the entry, if it's set.
    pub fn file_name(&self) -> Option<&str> {
        self.body.file_name()
    }

    /// Set the file name of the `Body`.
    pub fn set_file_name<P>(&mut self, file_name: Option<P>)
    where
        P: AsRef<str>,
    {
        self.body.set_file_name(file_name);
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("name", &self.name)
            .field("body", &self.body)
            .finish()
    }
}

impl AsyncRead for Entry {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.body).poll_read(cx, buf)
    }
}

impl AsyncBufRead for Entry {
    #[allow(missing_doc_code_examples)]
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    fn poll_fill_buf(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        // Pin::new(&mut self.body).poll_fill_buf(cx)
        todo!("Pin::new(&mut self.body).poll_fill_buf(cx)")
    }

    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        Pin::new(&mut self.body).consume(amt)
    }
}

impl AsRef<Body> for Entry {
    fn as_ref(&self) -> &Body {
        &self.body
    }
}

impl AsMut<Body> for Entry {
    fn as_mut(&mut self) -> &mut Body {
        &mut self.body
    }
}

impl Into<Body> for Entry {
    fn into(self) -> Body {
        self.body
    }
}

impl From<Body> for Entry {
    fn from(body: Body) -> Self {
        match body.file_name.clone() {
            Some(name) => Self { body, name },
            None => Self {
                body,
                name: String::new(),
            },
        }
    }
}
