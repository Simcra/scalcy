use std::collections::VecDeque;

use crate::{Lexer, Parser};

#[derive(Debug, Default)]
pub struct Calculator {
    input: String,
    result: String,
    history: VecDeque<String>,
}

impl Calculator {
    pub fn input(&self) -> &str {
        self.input.as_str()
    }

    pub fn result(&self) -> &str {
        self.result.as_str()
    }

    pub fn history(&self) -> &VecDeque<String> {
        &self.history
    }

    pub fn append(&mut self, input: &str) {
        if input
            .chars()
            .any(|c| (c.is_control() && c != '\n' && c != '\r') || c >= '\u{f700}')
        {
            return;
        }

        self.input.push_str(input)
    }

    pub fn pop(&mut self) -> Option<char> {
        self.input.pop()
    }

    pub fn store(&mut self) {
        if self.input.eq_ignore_ascii_case(&self.result) {
            return;
        }

        if self.history.len() >= 5 {
            self.history.pop_back();
        }
        let last_calculation = self.result.clone() + " = " + self.input.as_str();
        self.history.push_front(last_calculation);

        self.input = self.result.clone()
    }

    pub fn clear(&mut self) {
        self.input = String::from("0")
    }

    pub fn calculate(&mut self) {
        let mut lexer = Lexer::new(self.input.as_str());
        let lexer_result = lexer.lex();
        if lexer_result.is_err() {
            self.result = lexer_result.unwrap_err();
            return;
        }

        let mut parser = Parser::new(lexer_result.unwrap());
        let parser_result = parser.parse();
        if parser_result.is_err() {
            self.result = parser_result.unwrap_err();
            return;
        }

        // Do the calculation and update the GUI
        let expr = parser_result.unwrap();
        self.result = expr.solve().to_string()
    }
}
