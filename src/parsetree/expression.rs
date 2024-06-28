use crate::{BinaryOperator, Constant, UnaryOperator};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    Constant(Constant),
    UnaryPreExpression(UnaryOperator, Box<Expression>),
    UnaryPostExpression(UnaryOperator, Box<Expression>),
    BinaryExpression(BinaryOperator, Box<Expression>, Box<Expression>),
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Number(0.0)
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Number(value) => value.to_string(),
            Expression::Constant(value) => {
                format!("{:?}{{{}}}", value, f64::from(*value).to_string())
            }
            Expression::UnaryPreExpression(op, a) => {
                "( ".to_string() + op.to_string().as_str() + a.as_ref().to_string().as_str() + " )"
            }
            Expression::UnaryPostExpression(op, a) => {
                "( ".to_string() + a.as_ref().to_string().as_str() + op.to_string().as_str() + " )"
            }
            Expression::BinaryExpression(op, a, b) => {
                "( ".to_string()
                    + a.as_ref().to_string().as_str()
                    + " "
                    + op.to_string().as_str()
                    + " "
                    + b.as_ref().to_string().as_str()
                    + " )"
            }
        }
    }
}

impl From<Expression> for String {
    fn from(value: Expression) -> Self {
        value.to_string()
    }
}
