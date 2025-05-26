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

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => write!(f, "AND"),
            Self::Class => write!(f, "CLASS"),
            Self::Else => write!(f, "ELSE"),
            Self::False => write!(f, "FALSE"),
            Self::Fun => write!(f, "FUN"),
            Self::For => write!(f, "FOR"),
            Self::If => write!(f, "IF"),
            Self::Nil => write!(f, "NIL"),
            Self::Or => write!(f, "OR"),
            Self::Print => write!(f, "PRINT"),
            Self::Return => write!(f, "RETURN"),
            Self::Super => write!(f, "SUPER"),
            Self::This => write!(f, "THIS"),
            Self::True => write!(f, "TRUE"),
            Self::Var => write!(f, "VAR"),
            Self::While => write!(f, "WHILE"),
        }
    }
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

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operator(op) => write!(f, "{}", op),
            Self::Keyword(kw) => write!(f, "{}", kw),
            Self::Literal(lit) => write!(f, "{:?}", lit),
            Self::Invalid => write!(f, "Invalid"),
        }
    }
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
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add parsed value for literals
        write!(f, "{} {} null", self.ty, self.lexeme)
    }
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String) -> Self {
        Self { ty, lexeme }
    }

    pub fn ty(&self) -> &TokenType {
        &self.ty
    }
}
