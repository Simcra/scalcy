use crate::{BinaryOperator, Constant, UnaryPreOperator};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    Constant(Constant),
    UnaryPreExpression(UnaryPreOperator, Box<Expression>),
    // UnaryPostExpression(UnaryPostOperator, Box<Expression>),
    BinaryExpression(BinaryOperator, Box<Expression>, Box<Expression>),
}

impl Default for Expression {
    fn default() -> Self {
        Self::Number(0.0)
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Self::Number(value) => value.to_string(),
            Self::Constant(value) => {
                format!("{:?}{{{}}}", value, f64::from(*value).to_string())
            }
            Self::UnaryPreExpression(op, a) => {
                "( ".to_string() + op.to_string().as_str() + a.as_ref().to_string().as_str() + " )"
            }
            // Self::UnaryPostExpression(op, a) => {
            //     "( ".to_string() + a.as_ref().to_string().as_str() + op.to_string().as_str() + " )"
            // }
            Self::BinaryExpression(op, a, b) => {
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

impl Expression {
    pub fn solve(&self) -> f64 {
        match self {
            Self::Number(value) => value.clone(),
            Self::Constant(value) => value.clone().into(),
            Self::UnaryPreExpression(op, expr) => {
                let value = expr.solve();
                op.calculate(value)
            }
            Self::BinaryExpression(op, expr_a, expr_b) => {
                let value_a = expr_a.solve();
                let value_b = expr_b.solve();
                op.calculate(value_a, value_b)
            }
        }
    }
}
