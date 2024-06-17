#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
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

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "0" => Token::Zero,
            "1" => Token::One,
            "2" => Token::Two,
            "3" => Token::Three,
            "4" => Token::Four,
            "5" => Token::Five,
            "6" => Token::Six,
            "7" => Token::Seven,
            "8" => Token::Eight,
            "9" => Token::Nine,
            "(" => Token::OpenBracket,
            ")" => Token::CloseBracket,
            " mod " => Token::Modulo,
            "π" => Token::Pi,
            "/" => Token::Divide,
            "√" => Token::SquareRoot,
            "*" => Token::Multiply,
            "²" => Token::Square,
            "-" => Token::Subtract,
            " pow " => Token::Power,
            "." => Token::Period,
            "+" => Token::Add,
            "=" => Token::Equal,
            _ => Token::Unknown,
        }
    }
}

impl From<Token> for String {
    fn from(value: Token) -> String {
        match value {
            Token::Zero => "0",
            Token::One => "1",
            Token::Two => "2",
            Token::Three => "3",
            Token::Four => "4",
            Token::Five => "5",
            Token::Six => "6",
            Token::Seven => "7",
            Token::Eight => "8",
            Token::Nine => "9",
            Token::OpenBracket => "(",
            Token::CloseBracket => ")",
            Token::Modulo => " mod ",
            Token::Pi => "π",
            Token::Divide => "/",
            Token::SquareRoot => "√",
            Token::Multiply => "*",
            Token::Square => "²",
            Token::Subtract => "-",
            Token::Power => " pow ",
            Token::Period => ".",
            Token::Add => "+",
            Token::Equal => "=",
            Token::Unknown => "???",
        }
        .into()
    }
}
