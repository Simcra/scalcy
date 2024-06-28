use crate::{Lexer, Parser};

#[derive(Default)]
pub struct Calculator {
    input: String,
    result: String,
}

impl Calculator {
    pub fn input(&self) -> &str {
        return self.input.as_str();
    }

    pub fn result(&self) -> &str {
        return self.result.as_str();
    }

    pub fn append(&mut self, input: &str) {
        if input
            .chars()
            .any(|c| (c.is_control() && c != '\n' && c != '\r') || c >= '\u{f700}')
        {
            return;
        }

        self.input.push_str(input);
    }

    pub fn pop(&mut self) -> Option<char> {
        self.input.pop()
    }

    pub fn store(&mut self) {
        self.input = self.result.clone();
    }

    pub fn clear(&mut self) {
        self.input = String::from("0");
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

        self.result = "Result".to_string();
    }
}
