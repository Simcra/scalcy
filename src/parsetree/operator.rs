use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::{FromToken, Precedence, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryPreOperator {
    SquareRoot,
}

impl ToString for UnaryPreOperator {
    fn to_string(&self) -> String {
        match self {
            Self::SquareRoot => Token::SquareRoot.to_string(),
        }
    }
}

impl Precedence for UnaryPreOperator {
    fn precedence(&self) -> u8 {
        match self {
            Self::SquareRoot => Token::SquareRoot.precedence(),
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
            Token::SquareRoot => Ok(Self::SquareRoot),
            _ => {
                let token_str = token.to_string();
                Err(format!("Token \'{token_str}\' is not a unary pre-operator"))
            }
        }
    }
}

impl UnaryPreOperator {
    pub fn calculate(&self, value_rhs: f64) -> f64 {
        match self {
            Self::SquareRoot => value_rhs.sqrt(),
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
            Self::Add => Token::Add.to_string(),
            Self::Subtract => Token::Subtract.to_string(),
            Self::Multiply => Token::Multiply.to_string(),
            Self::Divide => Token::Divide.to_string(),
            Self::Power => Token::Power.to_string(),
            Self::Modulo => Token::Modulo.to_string(),
        }
    }
}

impl Precedence for BinaryOperator {
    fn precedence(&self) -> u8 {
        match self {
            Self::Add => Token::Add.precedence(),
            Self::Subtract => Token::Subtract.precedence(),
            Self::Multiply => Token::Multiply.precedence(),
            Self::Divide => Token::Divide.precedence(),
            Self::Power => Token::Power.precedence(),
            Self::Modulo => Token::Modulo.precedence(),
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
            Token::Add => Ok(Self::Add),
            Token::Subtract => Ok(Self::Subtract),
            Token::Multiply => Ok(Self::Multiply),
            Token::Divide => Ok(Self::Divide),
            Token::Power => Ok(Self::Power),
            Token::Modulo => Ok(Self::Modulo),
            _ => {
                let token_str = token.to_string();
                Err(format!("Token \'{token_str}\' is not a binary operator"))
            }
        }
    }
}

impl BinaryOperator {
    pub fn calculate(&self, value_lhs: f64, value_rhs: f64) -> f64 {
        match self {
            Self::Add => value_lhs.add(value_rhs),
            Self::Subtract => value_lhs.sub(value_rhs),
            Self::Multiply => value_lhs.mul(value_rhs),
            Self::Divide => value_lhs.div(value_rhs),
            Self::Power => value_lhs.powf(value_rhs),
            Self::Modulo => value_lhs.rem(value_rhs),
        }
    }
}
