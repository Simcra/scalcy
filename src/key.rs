use crate::token::Token;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Key {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Period,
    Percent,
    Caret,
    Asterisk,
    LeftParenthesis,
    RightParenthesis,
    Hyphen,
    Equal,
    Plus,
    Backspace,
    Delete,
    Enter,
    ForwardSlash,
    Unknown,
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        match value {
            "1" => Key::One,
            "2" => Key::Two,
            "3" => Key::Three,
            "4" => Key::Four,
            "5" => Key::Five,
            "6" => Key::Six,
            "7" => Key::Seven,
            "8" => Key::Eight,
            "9" => Key::Nine,
            "0" => Key::Zero,
            "." => Key::Period,
            "%" => Key::Percent,
            "^" => Key::Caret,
            "*" => Key::Asterisk,
            "(" => Key::LeftParenthesis,
            ")" => Key::RightParenthesis,
            "-" => Key::Hyphen,
            "=" => Key::Equal,
            "+" => Key::Plus,
            "\x08" => Key::Backspace,
            "\x7f" => Key::Delete,
            "\n" => Key::Enter,
            "\r" => Key::Enter,
            "\r\n" => Key::Enter,
            "/" => Key::ForwardSlash,
            _ => Key::Unknown,
        }
    }
}

impl From<Key> for String {
    fn from(value: Key) -> String {
        match value {
            Key::One => "1",
            Key::Two => "2",
            Key::Three => "3",
            Key::Four => "4",
            Key::Five => "5",
            Key::Six => "6",
            Key::Seven => "7",
            Key::Eight => "8",
            Key::Nine => "9",
            Key::Zero => "0",
            Key::Period => ".",
            Key::Percent => "%",
            Key::Caret => "^",
            Key::Asterisk => "*",
            Key::LeftParenthesis => "(",
            Key::RightParenthesis => ")",
            Key::Hyphen => "-",
            Key::Equal => "=",
            Key::Plus => "+",
            Key::Backspace => "Backspace",
            Key::Delete => "Delete",
            Key::Enter => "Enter",
            Key::ForwardSlash => "/",
            Key::Unknown => "???",
        }
        .into()
    }
}

impl From<Key> for Token {
    fn from(value: Key) -> Self {
        match value {
            Key::One => Token::One,
            Key::Two => Token::Two,
            Key::Three => Token::Three,
            Key::Four => Token::Four,
            Key::Five => Token::Five,
            Key::Six => Token::Six,
            Key::Seven => Token::Seven,
            Key::Eight => Token::Eight,
            Key::Nine => Token::Nine,
            Key::Zero => Token::Zero,
            Key::Period => Token::Period,
            Key::Percent => Token::Modulo,
            Key::Caret => Token::Power,
            Key::Asterisk => Token::Multiply,
            Key::LeftParenthesis => Token::OpenBracket,
            Key::RightParenthesis => Token::CloseBracket,
            Key::Hyphen => Token::Subtract,
            Key::Equal => Token::Equal,
            Key::Plus => Token::Add,
            Key::Enter => Token::Equal,
            Key::ForwardSlash => Token::Divide,
            _ => Token::Unknown,
        }
    }
}
