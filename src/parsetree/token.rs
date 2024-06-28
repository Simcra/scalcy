pub trait FromToken: Sized {
    type Err;
    fn from_token(token: &Token) -> Result<Self, Self::Err>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(f64),
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
    Add,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseTokenError;

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Number(value) => value.to_string(),
            Token::LeftParen => "(".to_string(),
            Token::RightParen => ")".to_string(),
            Token::Modulo => "%".to_string(),
            Token::Pi => "π".to_string(),
            Token::Divide => "/".to_string(),
            Token::SquareRoot => "√".to_string(),
            Token::Multiply => "*".to_string(),
            Token::Square => "²".to_string(),
            Token::Subtract => "-".to_string(),
            Token::Power => "^".to_string(),
            Token::Add => "+".to_string(),
        }
    }
}

impl From<Token> for String {
    fn from(value: Token) -> Self {
        value.to_string()
    }
}

impl Token {
    pub fn precedence(&self) -> u8 {
        match self {
            Token::LeftParen => 0,
            Token::RightParen => 0,
            Token::Modulo => 1,
            Token::Divide => 2,
            Token::SquareRoot => 1,
            Token::Multiply => 3,
            Token::Square => 1,
            Token::Subtract => 5,
            Token::Power => 1,
            Token::Add => 4,
            _ => 6,
        }
    }
}
