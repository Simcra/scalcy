use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::token::Token;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
    Pi,
    Square,
    SquareRoot,
    Unknown,
}

impl From<Token> for Operation {
    fn from(token: Token) -> Self {
        match token {
            Token::Add => Operation::Add,
            Token::Subtract => Operation::Subtract,
            Token::Multiply => Operation::Multiply,
            Token::Divide => Operation::Divide,
            Token::Power => Operation::Power,
            Token::Modulo => Operation::Modulo,
            Token::Pi => Operation::Pi,
            Token::Square => Operation::Square,
            Token::SquareRoot => Operation::SquareRoot,
            _ => Operation::Unknown,
        }
    }
}

impl Operation {
    fn perform(self, a: Option<f64>, b: Option<f64>) -> f64 {
        match self {
            Operation::Add => a.unwrap_or(0.0).add(b.unwrap_or(0.0)),
            Operation::Subtract => a.unwrap_or(0.0).sub(b.unwrap_or(0.0)),
            Operation::Multiply => {
                if a.is_none() && b.is_none() {
                    0.0
                } else {
                    a.unwrap_or(1.0).mul(b.unwrap_or(1.0))
                }
            }
            Operation::Divide => {
                if a.is_some() && b.is_some() {
                    a.unwrap().div(b.unwrap())
                } else if a.is_none() {
                    b.unwrap()
                } else {
                    a.unwrap()
                }
            }
            Operation::Power => {
                if a.is_some() && b.is_some() {
                    a.unwrap().powf(b.unwrap())
                } else if a.is_none() {
                    b.unwrap()
                } else {
                    a.unwrap()
                }
            }
            Operation::Modulo => {
                if a.is_some() && b.is_some() {
                    a.unwrap().rem(b.unwrap())
                } else if a.is_none() {
                    b.unwrap()
                } else {
                    a.unwrap()
                }
            }
            Operation::Pi => {
                if a.is_some() {
                    std::f64::consts::PI.mul(a.unwrap())
                } else {
                    std::f64::consts::PI
                }
            }
            Operation::Square => {
                if a.is_some() {
                    a.unwrap().powi(2)
                } else {
                    0.0
                }
            }
            Operation::SquareRoot => {
                if b.is_some() {
                    a.unwrap_or(1.0).mul(b.unwrap().sqrt())
                } else {
                    0.0
                }
            }
            Operation::Unknown => 0.0,
        }
    }
}
