use regex::Regex;
use std::borrow::Cow;

#[derive(Hash, PartialEq, Eq, Debug, Default)]
pub struct Txt(pub usize, pub Option<String>);

pub fn ps(text: &str) -> Option<Txt> {
    let p = spo(text, ".� ")?;
    Some(Txt(
        (p.0).parse::<usize>().unwrap_or(0),
        Some(p.1.to_string()),
    ))
}

pub fn spo<'a>(text: &'a str, pat: &'a str) -> Option<(&'a str, &'a str)> {
    text.split_once(pat)
}

pub fn re(text: &str) -> (Cow<'_, str>, Cow<'_, str>) {
    use lazy_static::lazy_static;

    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?x)
            (?P<n>\d+)    # 1
            (?P<d>\.\u{FFFD})    # .�
            (?P<p>.*)
            "#
        )
        .unwrap();
    }

    (RE.replace_all(text, "$n"), RE.replace_all(text, "$p"))
}

pub fn read_from_text() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ps() {
        let text = r#"1.� ./tests
11.� ./Cargo.lock"#;

        ps(text);
    }
}
