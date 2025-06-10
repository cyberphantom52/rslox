use std::error;

use crate::token::TokenType;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfInput,
    ParseError { msg: String },
    LexingError { ty: LexingError, line: usize },
}

#[derive(Debug)]
pub enum LexingError {
    UnterminatedString,
    UnexpectedCharacter(char),
    UnexpectedToken {
        expected: TokenType,
        found: TokenType,
    },
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingError::UnexpectedToken { expected, found } => {
                write!(f, "Unexpected Token: Expected {expected}, found {found}.")
            }
            LexingError::UnterminatedString => write!(f, "Unterminated string."),
            LexingError::UnexpectedCharacter(c) => write!(f, "Unexpected character: {c}"),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::UnexpectedEndOfInput => "Unexpected end of input".to_string(),
            Error::ParseError { msg } => format!("Parse error: {msg}"),
            Error::LexingError { ty, line } => {
                format!("[line {line}] Error: {}", ty)
            }
        };

        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}
