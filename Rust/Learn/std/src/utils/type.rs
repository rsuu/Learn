pub fn get_first_char(text: &str) -> char {
    text.chars().next().unwrap_or_default()
}

pub fn char_to_usize(text: char) -> usize {
    text as usize
}

pub fn text_regex() {
    use regex::Regex;
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("2014-01-01"));
}

pub fn hex_to_char(s: &str) -> Result<char, std::num::ParseIntError> {
    u8::from_str_radix(s, 16).map(|n| n as char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_char() {
        let v = vec!["00", "4b", "4c"];
        for s in v {
            println!("{:?}", hex_to_char(s));
        }
    }
}
