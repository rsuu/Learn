use crate::term;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Card {
    pub text: Vec<String>,
    pub size: term::TermSize,
}

impl Card {
    pub fn new() -> Self {
        Card {
            text: Vec::new(),
            size: term::TermSize::new(),
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}
