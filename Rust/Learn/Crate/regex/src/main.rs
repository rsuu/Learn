fn main() {}

pub fn re(text: &str) -> std::borrow::Cow<'_, str> {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<b>.*)").unwrap();
    }

    RE.replace_all(text, "ttt $b")
}
