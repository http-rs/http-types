use std::borrow::Cow;

/// <https://tools.ietf.org/html/rfc7230#section-3.2.6>
pub(crate) fn parse_token(input: &str) -> Option<(Cow<'_, str>, &str)> {
    let mut end_of_token = 0;
    for (i, c) in input.char_indices() {
        if tchar(c) {
            end_of_token = i + 1;
        } else {
            break;
        }
    }

    if end_of_token == 0 {
        None
    } else {
        let (token, rest) = input.split_at(end_of_token);
        Some((Cow::from(token), rest))
    }
}

/// <https://tools.ietf.org/html/rfc7230#section-3.2.6>
pub(crate) fn tchar(c: char) -> bool {
    matches!(
        c, 'a'..='z'
            | 'A'..='Z'
            | '0'..='9'
            | '!'
            | '#'
            | '$'
            | '%'
            | '&'
            | '\''
            | '*'
            | '+'
            | '-'
            | '.'
            | '^'
            | '_'
            | '`'
            | '|'
            | '~'
    )
}

/// <https://tools.ietf.org/html/rfc7230#section-3.2.6>
fn vchar(c: char) -> bool {
    matches!(c as u8, b'\t' | 32..=126 | 128..=255)
}

/// <https://tools.ietf.org/html/rfc7230#section-3.2.6>
pub(crate) fn parse_quoted_string(input: &str) -> Option<(Cow<'_, str>, &str)> {
    // quoted-string must start with a DQUOTE
    if !input.starts_with('"') {
        return None;
    }

    let mut end_of_string = None;
    let mut backslashes: Vec<usize> = vec![];

    for (i, c) in input.char_indices().skip(1) {
        if i > 1 && backslashes.last() == Some(&(i - 2)) {
            if !vchar(c) {
                // only VCHARs can be escaped
                return None;
            }
        // otherwise, we skip over this character while parsing
        } else {
            match c as u8 {
                // we have reached a quoted-pair
                b'\\' => {
                    backslashes.push(i - 1);
                }

                // end of the string, DQUOTE
                b'"' => {
                    end_of_string = Some(i + 1);
                    break;
                }

                // qdtext
                b'\t' | b' ' | 15 | 35..=91 | 93..=126 | 128..=255 => {}

                // unexpected character, bail
                _ => return None,
            }
        }
    }

    if let Some(end_of_string) = end_of_string {
        let value = &input[1..end_of_string - 1]; // strip DQUOTEs from start and end

        let value = if backslashes.is_empty() {
            // no backslashes means we don't need to allocate
            value.into()
        } else {
            backslashes.reverse(); // so that we can use pop. goes from low-to-high to high-to-low sorting

            value
                .char_indices()
                .filter_map(|(i, c)| {
                    if Some(&i) == backslashes.last() {
                        // they're already sorted highest to lowest, so we only need to check the last one
                        backslashes.pop();
                        None // remove the backslash from the output
                    } else {
                        Some(c)
                    }
                })
                .collect::<String>()
                .into()
        };

        Some((value, &input[end_of_string..]))
    } else {
        // we never reached a closing DQUOTE, so we do not have a valid quoted-string
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn token_successful_parses() {
        assert_eq!(parse_token("key=value"), Some(("key".into(), "=value")));
        assert_eq!(parse_token("KEY=value"), Some(("KEY".into(), "=value")));
        assert_eq!(parse_token("0123)=value"), Some(("0123".into(), ")=value")));
        assert_eq!(parse_token("a=b"), Some(("a".into(), "=b")));
        assert_eq!(
            parse_token("!#$%&'*+-.^_`|~=value"),
            Some(("!#$%&'*+-.^_`|~".into(), "=value"))
        );
    }

    #[test]
    fn token_unsuccessful_parses() {
        assert_eq!(parse_token(""), None);
        assert_eq!(parse_token("=value"), None);
        for c in r#"(),/:;<=>?@[\]{}"#.chars() {
            let s = c.to_string();
            assert_eq!(parse_token(&s), None);

            let s = format!("match{}rest", s);
            assert_eq!(
                parse_token(&s),
                Some(("match".into(), &*format!("{}rest", c)))
            );
        }
    }

    #[test]
    fn qstring_successful_parses() {
        assert_eq!(
            parse_quoted_string(r#""key"=value"#),
            Some((Cow::Borrowed("key"), "=value"))
        );

        assert_eq!(
            parse_quoted_string(r#""escaped \" quote \""rest"#),
            Some((Cow::Owned(String::from(r#"escaped " quote ""#)), r#"rest"#))
        );
    }

    #[test]
    fn qstring_unsuccessful_parses() {
        assert_eq!(parse_quoted_string(r#""abc"#), None);
        assert_eq!(parse_quoted_string(r#"hello""#), None);
        assert_eq!(parse_quoted_string(r#"=value\"#), None);
        assert_eq!(parse_quoted_string(r#"\""#), None);
        assert_eq!(parse_quoted_string(r#""\""#), None);
    }
}
