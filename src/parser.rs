#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Number(f64),
    // Variable(&str),
    LeftParen,
    RightParen,
    Modulo,
    Pi,
    Divide,
    SquareRoot,
    Multiply,
    Square,
    Subtract,
    Power,
    Period,
    Add,
    Equal,
}

#[derive(Debug, Clone)]
pub struct ParseTree {
    pub token: Token,
    pub children: Vec<ParseTree>,
}

impl ParseTree {
    pub fn new() -> ParseTree {
        ParseTree {
            token: Token::LeftParen,
            children: Vec::new(),
        }
    }
}
