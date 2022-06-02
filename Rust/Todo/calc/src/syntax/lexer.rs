use crate::syntax::{parser::get_number, tokens::LexItem};
use std::iter::Peekable;

pub fn lexer(input: &str) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();

    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = get_number(c, &mut it);
                result.push(LexItem::Num(n));
            }
            '+' | '-' | '*' | '/' | '^' | '_' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' => {
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(result)
}
