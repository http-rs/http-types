// Adapted from jsdom/whatwg-mimetype:
//
// - https://github.com/hyperium/mime/blob/8b04bcac22bb687b57704a7121b8c2765ed2dcaa/src/parse.rs
// - https://github.com/jsdom/whatwg-mimetype/blob/98408de520084336b4b17ec196a311e71d53e8e4/lib/parser.js

use omnom::prelude::*;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Cursor;

use super::Mime;

macro_rules! bail {
    ($fmt:expr) => {{
        let err = std::io::Error::new(std::io::ErrorKind::InvalidData, $fmt);
        return Err(crate::Error::from_io(err, crate::StatusCode::BadRequest));
    }};
}

/// Parse a string into a mime type.
#[allow(dead_code)]
pub(crate) fn parse(s: &str) -> crate::Result<Mime> {
    // parse the "type"
    //
    // ```txt
    // text/html; charset=utf-8;
    // ^^^^^
    // ```
    let mut s = Cursor::new(s);
    let mut base_type = vec![];
    let read = s.read_until(b'/', &mut base_type).unwrap();
    if read == 0 || read == 1 {
        bail!("mime must be a type followed by a slash");
    } else if let Some(b'/') = base_type.last() {
        base_type.pop();
    } else {
        bail!("mime must be a type followed by a slash");
    }
    validate_code_points(&base_type)?;

    // parse the "subtype"
    //
    // ```txt
    // text/html; charset=utf-8;
    //      ^^^^^
    // ```
    let mut sub_type = vec![];
    let read = s.read_until(b';', &mut sub_type).unwrap();
    if read == 0 {
        bail!("no subtype found");
    }
    if let Some(b';') = sub_type.last() {
        sub_type.pop();
    }
    validate_code_points(&sub_type)?;

    // instantiate our mime struct
    let basetype = String::from_utf8(base_type).unwrap();
    let subtype = String::from_utf8(sub_type).unwrap();
    let mut mime = Mime {
        essence: format!("{}/{}", &basetype, &subtype),
        basetype,
        subtype,
        parameters: None,
        static_essence: None,
        static_basetype: None,
        static_subtype: None,
    };

    // parse parameters into a hashmap
    //
    // ```txt
    // text/html; charset=utf-8;
    //           ^^^^^^^^^^^^^^^
    // ```
    loop {
        // Stop parsing if there's no more bytes to consume.
        if s.fill_buf().unwrap().len() == 0 {
            break;
        }

        // Trim any whitespace.
        //
        // ```txt
        // text/html; charset=utf-8;
        //           ^
        // ```
        s.skip_while(is_http_whitespace_char)?;

        // Get the param name.
        //
        // ```txt
        // text/html; charset=utf-8;
        //            ^^^^^^^
        // ```
        let mut param_name = vec![];
        s.read_while(&mut param_name, |b| b != b';' && b != b'=')?;
        validate_code_points(&param_name)?;
        let mut param_name = String::from_utf8(param_name).unwrap();
        param_name.make_ascii_lowercase();

        // Ignore param names without values.
        //
        // ```txt
        // text/html; charset=utf-8;
        //                   ^
        // ```
        let mut token = vec![0; 1];
        s.read_exact(&mut token).unwrap();
        if token[0] == b';' {
            continue;
        }

        // Get the param value.
        //
        // ```txt
        // text/html; charset=utf-8;
        //                    ^^^^^^
        // ```
        let mut param_value = vec![];
        s.read_until(b';', &mut param_value).unwrap();
        if let Some(b';') = param_value.last() {
            param_value.pop();
        }

        validate_code_points(&param_value)?;
        let mut param_value = String::from_utf8(param_value).unwrap();
        param_value.make_ascii_lowercase();

        // Insert attribute pair into hashmap.
        if let None = mime.parameters {
            mime.parameters = Some(HashMap::new());
        }
        mime.parameters
            .as_mut()
            .unwrap()
            .insert(param_name, param_value);
    }

    Ok(mime)
}

fn validate_code_points(buf: &[u8]) -> crate::Result<()> {
    let all = buf.iter().all(|b| match b {
        b'-' | b'!' | b'#' | b'$' | b'%' => true,
        b'&' | b'\'' | b'*' | b'+' | b'.' => true,
        b'^' | b'_' | b'`' | b'|' | b'~' => true,
        b'A'..=b'Z' => true,
        b'a'..=b'z' => true,
        b'0'..=b'9' => true,
        _ => false,
    });

    if all {
        Ok(())
    } else {
        bail!("invalid HTTP code points found in mime")
    }
}

fn is_http_whitespace_char(b: u8) -> bool {
    match b {
        b' ' | b'\t' | b'\n' | b'\r' => true,
        _ => false,
    }
}

#[test]
fn test() {
    let mime = parse("text/html").unwrap();
    assert_eq!(mime.basetype(), "text");
    assert_eq!(mime.subtype(), "html");

    // technically invalid mime, but allow anyway
    let mime = parse("text/html;").unwrap();
    assert_eq!(mime.basetype(), "text");
    assert_eq!(mime.subtype(), "html");

    let mime = parse("text/html; charset=utf-8").unwrap();
    assert_eq!(mime.basetype(), "text");
    assert_eq!(mime.subtype(), "html");
    assert_eq!(mime.param("charset"), Some(&"utf-8".to_string()));

    let mime = parse("text/html; charset=utf-8;").unwrap();
    assert_eq!(mime.basetype(), "text");
    assert_eq!(mime.subtype(), "html");
    assert_eq!(mime.param("charset"), Some(&"utf-8".to_string()));

    assert!(parse("text").is_err());
    assert!(parse("text/").is_err());
    assert!(parse("t/").is_err());
    assert!(parse("t/h").is_ok());
}
