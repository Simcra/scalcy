use std::str::FromStr;

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
    Equal,
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
            Token::Equal => "=".to_string(),
        }
    }
}

impl From<Token> for String {
    fn from(value: Token) -> Self {
        value.to_string()
    }
}

impl FromStr for Token {
    type Err = ParseTokenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(Token::LeftParen),
            ")" => Ok(Token::RightParen),
            "%" => Ok(Token::Modulo),
            "π" => Ok(Token::Pi),
            "/" => Ok(Token::Divide),
            "√" => Ok(Token::SquareRoot),
            "*" => Ok(Token::Multiply),
            "²" => Ok(Token::Square),
            "-" => Ok(Token::Subtract),
            "^" => Ok(Token::Power),
            "+" => Ok(Token::Add),
            "=" => Ok(Token::Equal),
            _ => Err(ParseTokenError),
        }
    }
}
