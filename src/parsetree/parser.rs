use std::{iter::Peekable, vec::IntoIter};

use crate::{BinaryOperator, Constant, Expression, FromToken, Precedence, Token, UnaryPreOperator};

#[derive(Debug)]
pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        self.parse_expression(0)
    }

    fn next_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.next_token() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(format!("Expected {:?} but found {:?}", expected, token)),
            None => Err(format!("Expected {:?} but reached end of input", expected)),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.next_token() {
            Some(Token::Number(value)) => Ok(Expression::Number(value)),
            Some(Token::Pi) => Ok(Expression::Constant(Constant::Pi)),
            Some(Token::LeftParen) => {
                let expr = self.parse_expression(0)?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Some(token) => {
                if let Ok(op) = UnaryPreOperator::from_token(&token) {
                    let rhs = self.parse_primary()?;
                    Ok(Expression::UnaryPreExpression(op, Box::new(rhs)))
                } else {
                    Err(format!("Unexpected token: {:?}", token))
                }
            }
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn parse_expression(&mut self, min_precedence: u8) -> Result<Expression, String> {
        let mut lhs = self.parse_primary()?;

        while let Some(token) = self.peek_token().cloned() {
            if let Ok(op) = UnaryPreOperator::from_token(&token) {
                if op.precedence() < min_precedence {
                    break; // Unary operators have higher precedence
                }
                self.next_token();
                let rhs = self.parse_expression(op.precedence())?;
                lhs = Expression::UnaryPreExpression(op, Box::new(rhs));
            } else if let Ok(op) = BinaryOperator::from_token(&token) {
                if op.precedence() < min_precedence {
                    break; // Binary operators have higher precedence
                }
                self.next_token();
                let rhs = self.parse_expression(op.precedence() + 1)?;
                lhs = Expression::BinaryExpression(op, Box::new(lhs), Box::new(rhs));
            } else {
                break;
            }
        }

        Ok(lhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;

    fn parse_and_assert(input: &str, expected_expr: Expression) {
        let tokens = Lexer::new(input).lex().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.parse().unwrap();
        assert_eq!(parsed_expr, expected_expr);
    }

    #[test]
    fn test_simple_addition() {
        parse_and_assert(
            "2 + 3",
            Expression::BinaryExpression(
                BinaryOperator::Add,
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Number(3.0)),
            ),
        );
    }

    #[test]
    fn test_complex_expression() {
        parse_and_assert(
            "2 + 3 * 4 - √9",
            Expression::BinaryExpression(
                BinaryOperator::Subtract,
                Box::new(Expression::BinaryExpression(
                    BinaryOperator::Add,
                    Box::new(Expression::Number(2.0)),
                    Box::new(Expression::BinaryExpression(
                        BinaryOperator::Multiply,
                        Box::new(Expression::Number(3.0)),
                        Box::new(Expression::Number(4.0)),
                    )),
                )),
                Box::new(Expression::UnaryPreExpression(
                    UnaryPreOperator::SquareRoot,
                    Box::new(Expression::Number(9.0)),
                )),
            ),
        );
    }

    #[test]
    fn test_parentheses() {
        parse_and_assert(
            "2 * (3 + 4)",
            Expression::BinaryExpression(
                BinaryOperator::Multiply,
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::BinaryExpression(
                    BinaryOperator::Add,
                    Box::new(Expression::Number(3.0)),
                    Box::new(Expression::Number(4.0)),
                )),
            ),
        );
    }

    #[test]
    fn test_operator_precedence() {
        parse_and_assert(
            "2 ^ 3 * 4 + 5 / √9",
            Expression::BinaryExpression(
                BinaryOperator::Add,
                Box::new(Expression::BinaryExpression(
                    BinaryOperator::Multiply,
                    Box::new(Expression::BinaryExpression(
                        BinaryOperator::Power,
                        Box::new(Expression::Number(2.0)),
                        Box::new(Expression::Number(3.0)),
                    )),
                    Box::new(Expression::Number(4.0)),
                )),
                Box::new(Expression::BinaryExpression(
                    BinaryOperator::Divide,
                    Box::new(Expression::Number(5.0)),
                    Box::new(Expression::UnaryPreExpression(
                        UnaryPreOperator::SquareRoot,
                        Box::new(Expression::Number(9.0)),
                    )),
                )),
            ),
        );
    }
}
