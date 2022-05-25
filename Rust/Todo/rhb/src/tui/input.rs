use unicode_width::UnicodeWidthStr;

enum InputMode {
    Normal,
    Editing,
}

struct Input {
    input: String,
    input_mode: InputMode,
    messages: Vec<String>,
}

impl Default for Input {
    fn default() -> Self {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}
