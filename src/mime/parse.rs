// Adapted from hyperium/mime and jsdom/whatwg-mimetype:
//
// - https://github.com/hyperium/mime/blob/8b04bcac22bb687b57704a7121b8c2765ed2dcaa/src/parse.rs
// - https://github.com/jsdom/whatwg-mimetype/blob/98408de520084336b4b17ec196a311e71d53e8e4/lib/parser.js

use std::io::{self, Error, ErrorKind};

use super::Mime;

/// Parse a string into a mime type.
#[allow(dead_code)]
pub(crate) fn parse(s: &str) -> io::Result<Mime> {
    // We don't strip leading + trailing `\r\n`s and whitespaces here because we assume
    // the header parser has already taken care of this.
    let s = s.trim();
    let mut cursor = 0;
    let str_len = s.len();

    // Parse the mime's "type"; this is everything before the `/`.
    // Values must be valid HTTP code points, and the "type" cannot be empty.
    for b in s.bytes() {
        if b == b'/' {
            break;
        } else if (cursor + 1) == str_len {
            return Err(Error::new(
                ErrorKind::Other,
                "a slash (/) was missing between the type and subtype",
            ));
        } else if !validate_code_point(b) {
            return Err(Error::new(
                ErrorKind::Other,
                "an invalid token was encountered",
            ));
        }
        cursor += 1;
    }

    // Ensure the "type" is not empty.
    if cursor == 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "an invalid token was encountered",
        ));
    }

    // Save the "type" and move the cursor to the next position.
    let basetype_cursor = cursor;
    let basetype = s[0..basetype_cursor].to_string();
    cursor += 1;

    // Parse the "subtype"
    for b in s[cursor..].bytes() {
        if b == b';' {
            break;
        } else if !validate_code_point(b) {
            return Err(Error::new(
                ErrorKind::Other,
                "an invalid token was encountered",
            ));
        }
        cursor += 1;
    }

    // Ensure the "subtype" is not empty.
    if cursor == basetype_cursor {
        return Err(Error::new(
            ErrorKind::Other,
            "an invalid token was encountered",
        ));
    }

    // Save the "type" and move the cursor to the next position.
    let subtype_cursor = cursor;
    let subtype = s[basetype_cursor..subtype_cursor].to_string();
    cursor += 1;

    let mut mime = Mime {
        essence: s[0..subtype_cursor].to_string(),
        static_essence: None,
        basetype,
        subtype,
        static_basetype: None,
        static_subtype: None,
        parameters: None,
    };

    panic!();
}

fn validate_code_point(b: u8) -> bool {
    match b {
        b'a'..=b'z'
        | b'A'..=b'Z'
        | b'0'..=b'9'
        | b'!'
        | b'#'
        | b'$'
        | b'%'
        | b'&'
        | b'\''
        | b'+'
        | b'-'
        | b'.'
        | b'^'
        | b'_'
        | b'`'
        | b'|'
        | b'~' => true,
        _ => false,
    }
}
