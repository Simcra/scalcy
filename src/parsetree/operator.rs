use crate::{FromToken, Precedence, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryPreOperator {
    SquareRoot,
}

impl ToString for UnaryPreOperator {
    fn to_string(&self) -> String {
        match self {
            UnaryPreOperator::SquareRoot => Token::SquareRoot.to_string(),
        }
    }
}

impl Precedence for UnaryPreOperator {
    fn precedence(&self) -> u8 {
        match self {
            UnaryPreOperator::SquareRoot => Token::SquareRoot.precedence(),
        }
    }
}

impl From<UnaryPreOperator> for String {
    fn from(value: UnaryPreOperator) -> Self {
        value.to_string()
    }
}

impl FromToken for UnaryPreOperator {
    type Err = String;
    fn from_token(token: &Token) -> Result<Self, Self::Err> {
        match token {
            Token::SquareRoot => Ok(UnaryPreOperator::SquareRoot),
            _ => {
                let token_str = token.to_string();
                Err(format!("Token \'{token_str}\' is not a unary pre-operator"))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
}

impl ToString for BinaryOperator {
    fn to_string(&self) -> String {
        match self {
            BinaryOperator::Add => Token::Add.to_string(),
            BinaryOperator::Subtract => Token::Subtract.to_string(),
            BinaryOperator::Multiply => Token::Multiply.to_string(),
            BinaryOperator::Divide => Token::Divide.to_string(),
            BinaryOperator::Power => Token::Power.to_string(),
            BinaryOperator::Modulo => Token::Modulo.to_string(),
        }
    }
}

impl Precedence for BinaryOperator {
    fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Add => Token::Add.precedence(),
            BinaryOperator::Subtract => Token::Subtract.precedence(),
            BinaryOperator::Multiply => Token::Multiply.precedence(),
            BinaryOperator::Divide => Token::Divide.precedence(),
            BinaryOperator::Power => Token::Power.precedence(),
            BinaryOperator::Modulo => Token::Modulo.precedence(),
        }
    }
}

impl From<BinaryOperator> for String {
    fn from(value: BinaryOperator) -> Self {
        value.to_string()
    }
}

impl FromToken for BinaryOperator {
    type Err = String;
    fn from_token(token: &Token) -> Result<Self, Self::Err> {
        match token {
            Token::Add => Ok(BinaryOperator::Add),
            Token::Subtract => Ok(BinaryOperator::Subtract),
            Token::Multiply => Ok(BinaryOperator::Multiply),
            Token::Divide => Ok(BinaryOperator::Divide),
            Token::Power => Ok(BinaryOperator::Power),
            Token::Modulo => Ok(BinaryOperator::Modulo),
            _ => {
                let token_str = token.to_string();
                Err(format!("Token \'{token_str}\' is not a binary operator"))
            }
        }
    }
}
