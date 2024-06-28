use std::{iter::Peekable, vec::IntoIter};

use crate::{BinaryOperator, Constant, Expression, FromToken, Token, UnaryOperator};

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
        self.parse_expression()
    }

    fn next_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        if let Some(token) = self.peek_token().cloned() {
            match token {
                Token::Number(value) => {
                    self.next_token();
                    Ok(Expression::Number(value))
                }
                Token::Pi => {
                    self.next_token();
                    Ok(Expression::Constant(Constant::Pi))
                }
                Token::LeftParen => {
                    self.next_token();
                    let expr = self.parse_expression()?;
                    if let Some(Token::RightParen) = self.next_token() {
                        Ok(expr)
                    } else {
                        let expr_str = expr.to_string();
                        Err(format!(
                            "Expected closing parenthesis for expression \'{expr_str}\'"
                        ))
                    }
                }
                _ => Err(format!("Unexpected token \'{}\'", token.to_string())),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        if let Some(op) = self.peek_token().cloned() {
            match op {
                Token::SquareRoot => {
                    self.next_token();
                    Ok(Expression::UnaryExpression(
                        UnaryOperator::SquareRoot,
                        Box::new(self.parse_unary()?),
                    ))
                }
                _ => {
                    let expr = self.parse_primary()?;
                    self.parse_unary_postfix(expr)
                }
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    fn parse_unary_postfix(&mut self, expr: Expression) -> Result<Expression, String> {
        if let Some(op) = self.peek_token() {
            match op {
                Token::Square => {
                    self.next_token();
                    self.parse_unary_postfix(Expression::UnaryExpression(
                        UnaryOperator::Square,
                        Box::new(expr),
                    ))
                }
                _ => Ok(expr),
            }
        } else {
            Ok(expr)
        }
    }

    fn parse_binary(&mut self, precedence: u8) -> Result<Expression, String> {
        let mut expr_lhs = self.parse_unary()?;

        while let Some(token) = self.peek_token().cloned() {
            let precendence_curr = token.precedence();
            if precendence_curr < precedence {
                break;
            }

            self.next_token();
            let mut expr_rhs = self.parse_unary()?;

            while let Some(op_next) = self.peek_token() {
                let precendence_next = op_next.precedence();
                if precendence_next > precendence_curr {
                    expr_rhs = self.parse_binary(precendence_next)?;
                } else {
                    break;
                }
            }

            let operator = BinaryOperator::from_token(&token);
            if operator.is_err() {
                return Err(operator.unwrap_err());
            }

            expr_lhs = Expression::BinaryExpression(
                operator.unwrap(),
                Box::new(expr_lhs),
                Box::new(expr_rhs),
            );
        }

        Ok(expr_lhs)
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_binary(0)
    }
}
