use std::error;

#[derive(Debug)]
pub enum Error {
    UnexpectedToken(String),
    LexingError { lexeme: String, line: usize },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::UnexpectedToken(s) => {
                format!("Unexpected token: {}", s)
            }
            Error::LexingError { lexeme, line } => {
                format!("[line {}]: Lexing error: {}", line, lexeme)
            }
        };

        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}
