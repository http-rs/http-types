use std::borrow::Cow;

use super::LanguageRange;

fn split_tag(input: &str) -> Option<(&str, &str)> {
    match input.find('-') {
        Some(pos) if pos <= 8 => {
            let (tag, rest) = input.split_at(pos);
            Some((tag, &rest[1..]))
        }
        Some(_) => None,
        None => (input.len() <= 8).then(|| (input, "")),
    }
}

// language-range   = (1*8ALPHA *("-" 1*8alphanum)) / "*"
// alphanum         = ALPHA / DIGIT
pub(crate) fn parse(input: &str) -> crate::Result<LanguageRange> {
    let mut tags = Vec::new();

    let (tag, mut input) = split_tag(input).ok_or_else(|| crate::format_err!("WIP error"))?;
    crate::ensure!(!tag.is_empty(), "Language tag should not be empty");
    crate::ensure!(
        tag.bytes()
            .all(|b| (b'a'..=b'z').contains(&b) || (b'A'..=b'Z').contains(&b)),
        "Language tag should be alpha"
    );
    tags.push(Cow::from(tag.to_string()));

    while !input.is_empty() {
        let (tag, rest) = split_tag(input).ok_or_else(|| crate::format_err!("WIP error"))?;
        crate::ensure!(!tag.is_empty(), "Language tag should not be empty");
        crate::ensure!(
            tag.bytes().all(|b| (b'a'..=b'z').contains(&b)
                || (b'A'..=b'Z').contains(&b)
                || (b'0'..=b'9').contains(&b)),
            "Language tag should be alpha numeric"
        );
        tags.push(Cow::from(tag.to_string()));
        input = rest;
    }

    Ok(LanguageRange { tags })
}

#[test]
fn test() {
    let range = parse("en").unwrap();
    assert_eq!(&range.tags, &["en"]);

    let range = parse("en-CA").unwrap();
    assert_eq!(&range.tags, &["en", "CA"]);

    let range = parse("zh-Hant-CN-x-private1-private2").unwrap();
    assert_eq!(
        &range.tags,
        &["zh", "Hant", "CN", "x", "private1", "private2"]
    );
}
