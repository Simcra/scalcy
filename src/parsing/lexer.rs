use std::{iter::Peekable, str::Chars};

use crate::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexTokenStreamError;

pub fn lex_tokenstream(tokenstream: &str) -> Result<Vec<Token>, LexTokenStreamError> {
    let mut result = Vec::new();
    let mut charstream: Peekable<Chars> = tokenstream.chars().peekable();

    while let Some(&c) = charstream.peek() {
        match c {
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&c) = charstream.peek() {
                    if c.is_digit(10) || c == '.' {
                        number.push(c);
                        charstream.next();
                    } else {
                        break;
                    }
                }
                let value = number.parse::<f64>();
                if value.is_err() {
                    println!(
                        "Failed to parse number \"{}\" in sequence \"{}\"",
                        number, tokenstream
                    );
                    return Err(LexTokenStreamError);
                }
                result.push(Token::Number(value.unwrap()));
            }
            _ => {
                if c.is_whitespace() {
                    charstream.next();
                    continue;
                }

                let token = String::from(c).parse::<Token>();
                if token.is_err() {
                    println!(
                        "Failed to lex token \"{}\" in sequence \"{}\"",
                        c, tokenstream
                    );
                    return Err(LexTokenStreamError);
                }
                result.push(token.unwrap());
                charstream.next();
            }
        }
    }

    return Ok(result);
}
