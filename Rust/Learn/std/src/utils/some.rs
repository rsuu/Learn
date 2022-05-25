pub struct Some {}

impl Some {
    pub fn last_one_of_number() {
        // Get the last digit of number
        let a: i32 = 987 % 10;
        let b: i32 = -1;

        assert_eq!(7, a.abs());
        assert_eq!(1, (b % 10).abs());
    }

    pub fn last_tow_of_string(n: usize) {
        let s = String::from("hello world");
        let last_two: Vec<char> = s.chars().rev().take(n).collect();

        assert_eq!("ld", last_two.iter().rev().collect::<String>());
        assert_eq!("ld", &s[s.len() - n..s.len()]);
    }

    pub fn first_tow_of_string(n: usize) {
        let s = String::from("hello world");
        assert_eq!("he", &s[0..n]);
    }

    pub fn cut_tow_of_string(n: usize) {
        let s = String::from("hello world");
        assert_eq!("llo wor", &s[n..s.len() - n]);
    }

    pub fn remove_whitespace_mut(s: &mut String) {
        s.retain(|c| !c.is_whitespace());
    }

    pub fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    pub fn push_char_to_string(c: char) -> String {
        let mut s = String::from("test");
        s.push(c);
        s
    }
}
