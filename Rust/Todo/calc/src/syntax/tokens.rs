#[derive(Debug, Clone)]
pub enum LexItem {
    Paren(char),
    Op(char),
    Num(u64),
}
