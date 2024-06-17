use crate::token::Token;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Button {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Clear,
    OpenBracket,
    CloseBracket,
    Modulo,
    Pi,
    Divide,
    SquareRoot,
    Multiply,
    Square,
    Subtract,
    Power,
    Period,
    Add,
    Equal,
    Unknown,
}

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        match value {
            "0" => Button::Zero,
            "1" => Button::One,
            "2" => Button::Two,
            "3" => Button::Three,
            "4" => Button::Four,
            "5" => Button::Five,
            "6" => Button::Six,
            "7" => Button::Seven,
            "8" => Button::Eight,
            "9" => Button::Nine,
            "C" => Button::Clear,
            "(" => Button::OpenBracket,
            ")" => Button::CloseBracket,
            "mod" => Button::Modulo,
            "π" => Button::Pi,
            "÷" => Button::Divide,
            "√" => Button::SquareRoot,
            "×" => Button::Multiply,
            "x²" => Button::Square,
            "-" => Button::Subtract,
            "xʸ" => Button::Power,
            "." => Button::Period,
            "+" => Button::Add,
            "=" => Button::Equal,
            _ => Button::Unknown,
        }
    }
}

impl From<Button> for String {
    fn from(value: Button) -> String {
        match value {
            Button::Zero => "0",
            Button::One => "1",
            Button::Two => "2",
            Button::Three => "3",
            Button::Four => "4",
            Button::Five => "5",
            Button::Six => "6",
            Button::Seven => "7",
            Button::Eight => "8",
            Button::Nine => "9",
            Button::Clear => "C",
            Button::OpenBracket => "(",
            Button::CloseBracket => ")",
            Button::Modulo => "mod",
            Button::Pi => "π",
            Button::Divide => "÷",
            Button::SquareRoot => "√",
            Button::Multiply => "×",
            Button::Square => "x²",
            Button::Subtract => "-",
            Button::Power => "xʸ",
            Button::Period => ".",
            Button::Add => "+",
            Button::Equal => "=",
            Button::Unknown => "???",
        }
        .into()
    }
}

impl From<Button> for Token {
    fn from(value: Button) -> Token {
        match value {
            Button::Zero => Token::Zero,
            Button::One => Token::One,
            Button::Two => Token::Two,
            Button::Three => Token::Three,
            Button::Four => Token::Four,
            Button::Five => Token::Five,
            Button::Six => Token::Six,
            Button::Seven => Token::Seven,
            Button::Eight => Token::Eight,
            Button::Nine => Token::Nine,
            Button::OpenBracket => Token::OpenBracket,
            Button::CloseBracket => Token::CloseBracket,
            Button::Modulo => Token::Modulo,
            Button::Pi => Token::Pi,
            Button::Divide => Token::Divide,
            Button::SquareRoot => Token::SquareRoot,
            Button::Multiply => Token::Multiply,
            Button::Square => Token::Square,
            Button::Subtract => Token::Subtract,
            Button::Power => Token::Power,
            Button::Period => Token::Period,
            Button::Add => Token::Add,
            Button::Equal => Token::Equal,
            Button::Clear => Token::Unknown,
            Button::Unknown => Token::Unknown,
        }
    }
}
