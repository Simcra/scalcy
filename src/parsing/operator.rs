use crate::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Square,
    SquareRoot,
}

impl ToString for UnaryOperator {
    fn to_string(&self) -> String {
        match self {
            UnaryOperator::Square => Token::Square.to_string(),
            UnaryOperator::SquareRoot => Token::SquareRoot.to_string(),
        }
    }
}

impl From<UnaryOperator> for String {
    fn from(value: UnaryOperator) -> Self {
        value.to_string()
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

impl From<BinaryOperator> for String {
    fn from(value: BinaryOperator) -> Self {
        value.to_string()
    }
}
