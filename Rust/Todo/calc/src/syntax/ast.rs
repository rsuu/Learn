#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Node>,
    pub entry: GrammarItem,
}

// parser
#[derive(Debug, Clone)]
pub enum GrammarItem {
    Add,         // '+'
    Sub,         // '-'
    Mul,         // '*'
    Div,         // '/'
    Number(u64), // '0 - 9'
    Paren,       // '()'
    NthRoot,     // '_'
    NthPower,    // '^'
}

impl Node {
    pub fn new() -> Node {
        Node {
            children: Vec::new(),
            entry: GrammarItem::Paren,
        }
    }
}
