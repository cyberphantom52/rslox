use super::operator::*;

pub enum TokenType {
    Keyword,
    Literal,
    Operator(Operator),
    Invalid,
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        if let Ok(op) = Operator::try_from(value) {
            Self::Operator(op)
        } else {
            Self::Invalid
        }
    }
}

pub struct Token {
    ty: TokenType,
    lexeme: String,
    line_number: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, line_number: usize) -> Self {
        Self {
            ty,
            lexeme,
            line_number,
        }
    }

    pub fn invalid(lexeme: String, line: usize) -> Self {
        Self::new(TokenType::Invalid, lexeme, line)
    }
}
