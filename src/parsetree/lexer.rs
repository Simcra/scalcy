use std::{iter::Peekable, str::Chars};

use crate::Token;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(tokens_str: &'a str) -> Self {
        Self {
            chars: tokens_str.chars().peekable(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut result = Vec::new();

        while let Some(&ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.next_char();
                continue;
            }

            match ch {
                '0'..='9' => {
                    let number = self.lex_number()?;
                    result.push(Token::Number(number));
                }
                _ => {
                    let token = self.lex_token()?;
                    result.push(token);
                }
            }
        }

        Ok(result)
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn lex_number(&mut self) -> Result<f64, String> {
        let mut number = String::new();
        while let Some(&ch) = self.peek_char() {
            if ch.is_digit(10) || ch == '.' {
                number.push(ch);
                self.next_char();
            } else {
                break;
            }
        }

        match number.parse::<f64>() {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Failed to lex number '{}'", number)),
        }
    }

    fn lex_token(&mut self) -> Result<Token, String> {
        if let Some(ch) = self.next_char() {
            match ch {
                '(' => Ok(Token::LeftParen),
                ')' => Ok(Token::RightParen),
                '%' => Ok(Token::Modulo),
                'π' => Ok(Token::Pi),
                '/' => Ok(Token::Divide),
                '√' => Ok(Token::SquareRoot),
                '*' => Ok(Token::Multiply),
                '-' => Ok(Token::Subtract),
                '^' => Ok(Token::Power),
                '+' => Ok(Token::Add),
                _ => Err(format!("Failed to lex token '{}'", ch)),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Token;

    #[test]
    fn test_lex_numbers() {
        let mut lexer = Lexer::new("123");
        assert_eq!(lexer.lex_number().unwrap(), 123.0);
        lexer = Lexer::new("456.789");
        assert_eq!(lexer.lex_number().unwrap(), 456.789);
    }

    #[test]
    fn test_lex_tokens() {
        let mut lexer = Lexer::new("(");
        assert_eq!(lexer.lex_token().unwrap(), Token::LeftParen);
        lexer = Lexer::new(")");
        assert_eq!(lexer.lex_token().unwrap(), Token::RightParen);
        lexer = Lexer::new("%");
        assert_eq!(lexer.lex_token().unwrap(), Token::Modulo);
        lexer = Lexer::new("π");
        assert_eq!(lexer.lex_token().unwrap(), Token::Pi);
        lexer = Lexer::new("/");
        assert_eq!(lexer.lex_token().unwrap(), Token::Divide);
        lexer = Lexer::new("√");
        assert_eq!(lexer.lex_token().unwrap(), Token::SquareRoot);
        lexer = Lexer::new("*");
        assert_eq!(lexer.lex_token().unwrap(), Token::Multiply);
        lexer = Lexer::new("-");
        assert_eq!(lexer.lex_token().unwrap(), Token::Subtract);
        lexer = Lexer::new("^");
        assert_eq!(lexer.lex_token().unwrap(), Token::Power);
        lexer = Lexer::new("+");
        assert_eq!(lexer.lex_token().unwrap(), Token::Add);
    }

    #[test]
    fn test_lex_complex_expression() {
        let mut lexer = Lexer::new("123 + 456 * (789 - π)");

        let tokens = lexer.lex().unwrap();
        let expected_tokens = vec![
            Token::Number(123.0),
            Token::Add,
            Token::Number(456.0),
            Token::Multiply,
            Token::LeftParen,
            Token::Number(789.0),
            Token::Subtract,
            Token::Pi,
            Token::RightParen,
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_lex_more_complex_expression() {
        let mut lexer = Lexer::new("1 + 1.236 + (3 - (1 - 2)) * 47 / π + √42 - 3^3");

        let tokens = lexer.lex().unwrap();
        let expected_tokens = vec![
            Token::Number(1.0),
            Token::Add,
            Token::Number(1.236),
            Token::Add,
            Token::LeftParen,
            Token::Number(3.0),
            Token::Subtract,
            Token::LeftParen,
            Token::Number(1.0),
            Token::Subtract,
            Token::Number(2.0),
            Token::RightParen,
            Token::RightParen,
            Token::Multiply,
            Token::Number(47.0),
            Token::Divide,
            Token::Pi,
            Token::Add,
            Token::SquareRoot,
            Token::Number(42.0),
            Token::Subtract,
            Token::Number(3.0),
            Token::Power,
            Token::Number(3.0),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_lex_whitespace() {
        let mut lexer = Lexer::new("   123   +   456   ");

        let tokens = lexer.lex().unwrap();
        let expected_tokens = vec![Token::Number(123.0), Token::Add, Token::Number(456.0)];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_lex_invalid_input() {
        let mut lexer = Lexer::new("123 $ 456");

        let result = lexer.lex();
        assert!(result.is_err());
    }

    #[test]
    fn test_lex_empty_input() {
        let mut lexer = Lexer::new("");

        let tokens = lexer.lex().unwrap();
        let expected_tokens: Vec<Token> = vec![];

        assert_eq!(tokens, expected_tokens);
    }
}
