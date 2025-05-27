use std::error;

#[derive(Debug)]
pub enum Error {
    ParseError { msg: String },
    LexingError { ty: LexingError, line: usize },
}

#[derive(Debug)]
pub enum LexingError {
    UnterminatedString,
    UnexpectedCharacter(char),
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingError::UnterminatedString => write!(f, "Unterminated string literal"),
            LexingError::UnexpectedCharacter(c) => write!(f, "Unexpected character: {c}"),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::ParseError { msg } => format!("Parse error: {msg}"),
            Error::LexingError { ty, line } => {
                format!("[line {line}] Error: {}", ty)
            }
        };

        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}
