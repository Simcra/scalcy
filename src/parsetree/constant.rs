use crate::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constant {
    Pi,
}

impl ToString for Constant {
    fn to_string(&self) -> String {
        match self {
            Self::Pi => Token::Pi.to_string(),
        }
    }
}

impl From<Constant> for String {
    fn from(value: Constant) -> Self {
        value.to_string()
    }
}

impl From<Constant> for f64 {
    fn from(value: Constant) -> Self {
        match value {
            Constant::Pi => std::f64::consts::PI,
        }
    }
}
