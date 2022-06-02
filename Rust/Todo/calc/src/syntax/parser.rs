use crate::syntax::{
    ast::{GrammarItem, Node},
    lexer::lexer,
    tokens::LexItem,
};
use std::iter::Peekable;

pub fn parser(input: &str) -> Result<Node, String> {
    let tokens = lexer(input)?;

    parse_a(&tokens, 0).and_then(|(n, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err(format!(
                "Expected end of input, found {:?} at {}",
                tokens[i], i
            ))
        }
    })
}

pub fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u64 {
    let mut number = c
        .to_string()
        .parse::<u64>()
        .expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u64>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

// 1
pub fn parse_a(tokens: &[LexItem], pos: usize) -> Result<(Node, usize), String> {
    let (node_summand, pos) = parse_b(tokens, pos)?; // parse Mul/Div first
    let c = tokens.get(pos);

    match c {
        Some(&LexItem::Op('+')) => {
            // recurse on the expr
            let mut sum = Node::new();
            sum.entry = GrammarItem::Add;
            sum.children.push(node_summand);
            let (rhs, i) = parse_a(tokens, pos + 1)?; // loop
            sum.children.push(rhs);
            Ok((sum, i))
        }

        Some(&LexItem::Op('-')) => {
            // recurse on the expr
            let mut sum = Node::new();
            sum.entry = GrammarItem::Sub;
            sum.children.push(node_summand);
            let (rhs, i) = parse_a(tokens, pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, i))
        }

        _ => {
            // doing nothing
            Ok((node_summand, pos))
        }
    }
}

// 2
pub fn parse_b(tokens: &[LexItem], pos: usize) -> Result<(Node, usize), String> {
    let (node_term, pos) = parse_c(tokens, pos)?; // parse Nth first
    let c = tokens.get(pos);

    match c {
        Some(&LexItem::Op('*')) => {
            // recurse on the summand
            let mut product = Node::new();
            product.entry = GrammarItem::Mul;
            product.children.push(node_term);
            let (rhs, i) = parse_b(tokens, pos + 1)?;
            product.children.push(rhs);
            Ok((product, i))
        }

        Some(&LexItem::Op('/')) => {
            let mut product = Node::new();
            product.entry = GrammarItem::Div;
            product.children.push(node_term);
            let (rhs, i) = parse_b(tokens, pos + 1)?;
            product.children.push(rhs);
            Ok((product, i))
        }
        _ => {
            // doing nothing
            // parse + OR - next
            Ok((node_term, pos))
        }
    }
}

// 3
pub fn parse_c(tokens: &[LexItem], pos: usize) -> Result<(Node, usize), String> {
    let (node_term, pos) = parse_expr(tokens, pos)?;
    let c = tokens.get(pos);

    match c {
        // 1^2
        Some(&LexItem::Op('^')) => {
            // recurse on the summand
            let mut product = Node::new();
            product.entry = GrammarItem::NthPower;
            product.children.push(node_term);
            let (rhs, i) = parse_c(tokens, pos + 1)?;
            product.children.push(rhs);
            Ok((product, i))
        }

        // 1_2
        Some(&LexItem::Op('_')) => {
            let mut product = Node::new();
            product.entry = GrammarItem::NthPower;
            product.children.push(node_term);
            let (rhs, i) = parse_c(tokens, pos + 1)?;
            product.children.push(rhs);
            Ok((product, i))
        }
        _ => {
            // doing nothing
            // parse * OR / next
            Ok((node_term, pos))
        }
    }
}

// 4
pub fn parse_expr(tokens: &[LexItem], pos: usize) -> Result<(Node, usize), String> {
    let c: &LexItem = tokens
        .get(pos)
        .ok_or_else(|| String::from("Unexpected end of input, expected paren or number"))?;

    match *c {
        LexItem::Num(n) => {
            let mut node = Node::new();
            node.entry = GrammarItem::Number(n);
            Ok((node, pos + 1))
        }
        LexItem::Paren(c) => {
            match c {
                '(' | '[' | '{' => {
                    parse_a(tokens, pos + 1).and_then(|(node, pos)| {
                        if let Some(&LexItem::Paren(c2)) = tokens.get(pos) {
                            if c2 == matching(c) {
                                // okay!
                                let mut paren = Node::new();
                                paren.children.push(node);
                                Ok((paren, pos + 1))
                            } else {
                                Err(format!(
                                    "Expected {} but found {} at {}",
                                    matching(c),
                                    c2,
                                    pos
                                ))
                            }
                        } else {
                            Err(format!(
                                "Expected closing paren at {} but found {:?}",
                                pos,
                                tokens.get(pos)
                            ))
                        }
                    })
                }
                _ => Err(format!("Expected paren at {} but found {:?}", pos, c)),
            }
        }
        _ => Err(format!(
            "Unexpected token {:?}, expected paren or number",
            c
        )),
    }
}

pub fn matching(c: char) -> char {
    match c {
        ')' => '(',
        '(' => ')',
        ']' => '[',
        '[' => ']',
        '}' => '{',
        '{' => '}',
        _ => panic!("should have been a parenthesis!"),
    }
}
