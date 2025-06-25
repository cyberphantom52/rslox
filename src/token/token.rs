use std::borrow::Cow;

use crate::error::{Error, LexingError, LexingErrorKind};

use super::operator::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Literal {
    Identifier,
    String,
    Number(f64),
}

impl TryFrom<&str> for Literal {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with('"') {
            if value.ends_with('"') {
                Ok(Literal::String)
            } else {
                return Err(Error::LexingError(LexingError::new(
                    LexingErrorKind::UnterminatedString,
                )));
            }
        } else if value.chars().all(|c| c.is_ascii_digit() || c == '.') {
            Ok(Literal::Number(value.parse::<f64>().unwrap()))
        } else {
            let starts_with_number = value.chars().next().map_or(false, |c| c.is_ascii_digit());

            if !starts_with_number && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return Ok(Literal::Identifier);
            }

            Err(Error::LexingError(LexingError::new(
                LexingErrorKind::InvalidLiteral(value.to_string()),
            )))
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier => write!(f, "IDENTIFIER"),
            Self::String => write!(f, "STRING"),
            Self::Number(_) => write!(f, "NUMBER"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    And,
    Or,

    If,
    Else,

    True,
    False,

    For,
    While,

    Class,
    Fun,
    Var,

    Print,
    Return,

    Super,
    This,

    Nil,
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
            _ => Err(Error::LexingError(LexingError::new(
                LexingErrorKind::InvalidKeyword(value.to_string()),
            ))),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
            Self::Literal(lit) => write!(f, "{}", lit),
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
        } else if let Ok(lit) = Literal::try_from(value) {
            Self::Literal(lit)
        } else {
            Self::Invalid
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token<'a> {
    ty: TokenType,
    lexeme: &'a str,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add parsed value for literals
        match &self.ty {
            TokenType::Literal(lit) => match lit {
                Literal::Identifier => write!(f, "{} {} null", self.ty, self.lexeme),
                Literal::String => write!(
                    f,
                    "{} {} {}",
                    self.ty,
                    self.lexeme,
                    self.lexeme.trim_matches('"')
                ),
                Literal::Number(num) => write!(
                    f,
                    "{} {} {}",
                    self.ty,
                    self.lexeme,
                    if num.fract() == 0f64 {
                        format!("{}.0", num)
                    } else {
                        format!("{}", num)
                    }
                ),
            },
            _ => write!(f, "{} {} null", self.ty, self.lexeme),
        }
    }
}

impl<'a> Token<'a> {
    pub fn new(ty: TokenType, lexeme: &'a str) -> Self {
        Self { ty, lexeme }
    }

    pub fn ty(&self) -> TokenType {
        self.ty
    }

    pub fn lexeme(&self) -> &'a str {
        self.lexeme
    }

    pub fn unescape(s: &'a str) -> Cow<'a, str> {
        Cow::Borrowed(s.trim_matches('"'))
    }
}
