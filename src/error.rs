use std::error;

use crate::token::{Keyword, Operator, TokenType};

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfInput,
    ParseError(ParseError),
    LexingError(LexingError),
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    line: Option<usize>,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind) -> Self {
        Self { kind, line: None }
    }

    pub fn kind(&self) -> &ParseErrorKind {
        &self.kind
    }

    pub fn line(&self) -> Option<usize> {
        self.line
    }

    pub fn with_line(kind: ParseErrorKind, line: usize) -> Self {
        Self {
            kind,
            line: Some(line),
        }
    }
}

#[derive(Debug)]
pub enum ParseErrorKind {
    UnsupportedOperator(Operator),
    UnsupportedKeyword(Keyword),
    UnexpectedOperator(Operator),
    UnexpectedKeyword(Keyword),
    UnexpectedToken(TokenType, String),
    InvalidExpression(String),
}

impl std::fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::UnsupportedOperator(op) => write!(f, "Unsupported Operator: {op}"),
            ParseErrorKind::UnsupportedKeyword(kw) => write!(f, "Unsupported Keyword: {kw}"),
            ParseErrorKind::UnexpectedOperator(op) => write!(f, "Unexpected Operator: {op}"),
            ParseErrorKind::UnexpectedKeyword(kw) => write!(f, "Unexpected Keyword: {kw}"),
            ParseErrorKind::UnexpectedToken(ty, lexeme) => {
                write!(f, "Unexpected token: {ty} lexeme: {lexeme}")
            }
            ParseErrorKind::InvalidExpression(lexeme) => {
                write!(f, "Error at '{lexeme}': Expect expression.")
            }
        }
    }
}

#[derive(Debug)]
pub struct LexingError {
    kind: LexingErrorKind,
    line: Option<usize>,
}

impl LexingError {
    pub fn new(kind: LexingErrorKind) -> Self {
        Self { kind, line: None }
    }

    pub fn kind(&self) -> &LexingErrorKind {
        &self.kind
    }

    pub fn line(&self) -> Option<usize> {
        self.line
    }

    pub fn with_line(kind: LexingErrorKind, line: usize) -> Self {
        Self {
            kind,
            line: Some(line),
        }
    }
}

#[derive(Debug)]
pub enum LexingErrorKind {
    InvalidOperator(String),
    InvalidLiteral(String),
    InvalidKeyword(String),
    UnterminatedString,
    UnexpectedCharacter(char),
    UnexpectedToken {
        expected: TokenType,
        found: TokenType,
    },
}

impl std::fmt::Display for LexingErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingErrorKind::InvalidKeyword(kw) => write!(f, "Invalid Keyword {kw}"),
            LexingErrorKind::InvalidLiteral(lit) => write!(f, "Invalid Literal {lit}"),
            LexingErrorKind::InvalidOperator(op) => write!(f, "Invalid Operator: {op}."),
            LexingErrorKind::UnexpectedToken { expected, found } => {
                write!(f, "Unexpected Token: Expected {expected}, found {found}.")
            }
            LexingErrorKind::UnterminatedString => write!(f, "Unterminated string."),
            LexingErrorKind::UnexpectedCharacter(c) => write!(f, "Unexpected character: {c}"),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::UnexpectedEndOfInput => "Unexpected end of input".to_string(),
            Error::ParseError(e) => {
                if e.line.is_some() {
                    format!("[line {}] {}", e.line.unwrap(), e.kind)
                } else {
                    format!("{}", e.kind)
                }
            }
            Error::LexingError(e) => {
                if e.line.is_some() {
                    format!("[line {}] Error: {}", e.line.unwrap(), e.kind)
                } else {
                    format!("Error: {}", e.kind)
                }
            }
        };

        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}
