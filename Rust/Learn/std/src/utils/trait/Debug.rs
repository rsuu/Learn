struct Words<'words> {
    pub len: usize,
    pub head_words: &'words str,
    pub head_char: char,
}

impl std::fmt::Debug for Words<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.len)
            .field(&self.head_words)
            .field(&self.head_char)
            .finish()
    }
}
impl<'words> Words<'words> {
    fn new(text: &'words str) -> Self {
        Words {
            len: text.len(),
            head_words: text,
            head_char: get_first_char(text),
        }
    }
}

fn main() {

}

