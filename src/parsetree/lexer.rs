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
            match ch {
                '0'..='9' => {
                    let number = self.lex_number()?;
                    result.push(Token::Number(number));
                }
                _ => {
                    while let Some(&ch) = self.peek_char() {
                        if ch.is_whitespace() {
                            self.next_char();
                        } else {
                            break;
                        }
                    }

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
                '²' => Ok(Token::Square),
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
