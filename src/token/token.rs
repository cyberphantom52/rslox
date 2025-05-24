use crate::error::Error;

use super::operator::*;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Identifier,
    String,
    Number,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl TryFrom<&str> for Keyword {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "and" => Ok(Keyword::And),
            "class" => Ok(Keyword::Class),
            "else" => Ok(Keyword::Else),
            "false" => Ok(Keyword::False),
            "fun" => Ok(Keyword::Fun),
            "for" => Ok(Keyword::For),
            "if" => Ok(Keyword::If),
            "nil" => Ok(Keyword::Nil),
            "or" => Ok(Keyword::Or),
            "print" => Ok(Keyword::Print),
            "return" => Ok(Keyword::Return),
            "super" => Ok(Keyword::Super),
            "this" => Ok(Keyword::This),
            "true" => Ok(Keyword::True),
            "var" => Ok(Keyword::Var),
            "while" => Ok(Keyword::While),
            _ => Err(Error::UnexpectedToken(value.to_string())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Keyword(Keyword),
    Literal(Literal),
    Operator(Operator),
    Invalid,
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        if let Ok(op) = Operator::try_from(value) {
            Self::Operator(op)
        } else if let Ok(kw) = Keyword::try_from(value) {
            Self::Keyword(kw)
        } else {
            Self::Invalid
        }
    }
}

#[derive(Debug)]
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
}
