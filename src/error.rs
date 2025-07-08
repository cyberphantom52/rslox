use crate::token::{Keyword, Operator, TokenType};
use miette::{Diagnostic, SourceSpan};
use std::error;

pub type ParseError = DiagnosticError<ParseErrorKind>;
pub type LexingError = DiagnosticError<LexingErrorKind>;
pub type RuntimeError = DiagnosticError<RuntimeErrorKind>;

#[derive(Debug, Diagnostic)]
pub enum Error {
    UnexpectedEndOfInput,

    #[diagnostic(transparent)]
    ParseError(ParseError),

    #[diagnostic(transparent)]
    LexingError(LexingError),

    #[diagnostic(transparent)]
    RuntimeError(RuntimeError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::UnexpectedEndOfInput => "Unexpected end of input".to_string(),
            Error::ParseError(e) => format!("{}", e),
            Error::LexingError(e) => format!("{}", e),
            Error::RuntimeError(e) => format!("{}", e),
        };

        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}

// Generic diagnostic error that can be used for all error types
#[derive(Debug, Diagnostic)]
pub struct DiagnosticError<K: std::fmt::Debug + std::fmt::Display> {
    #[source_code]
    source: String,

    kind: K,

    #[label = "here"]
    span: SourceSpan,
}

impl<K> std::error::Error for DiagnosticError<K> where K: std::fmt::Debug + std::fmt::Display {}

impl<K> std::fmt::Display for DiagnosticError<K>
where
    K: std::fmt::Debug + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl<K: std::fmt::Debug + std::fmt::Display> DiagnosticError<K> {
    pub fn new(source: String, kind: K, span: SourceSpan) -> Self {
        Self { source, kind, span }
    }

    pub fn kind(&self) -> &K {
        &self.kind
    }

    pub fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    UnsupportedOperator(Operator),
    UnsupportedKeyword(Keyword),
    UnexpectedOperator(Operator),
    UnexpectedKeyword(Keyword),
    UnexpectedToken(TokenType, String),
    InvalidExpression(String),
}

impl std::error::Error for ParseErrorKind {}

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

#[derive(Debug, Clone)]
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

impl std::error::Error for LexingErrorKind {}

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

#[derive(Debug, Clone)]
pub enum RuntimeErrorKind {
    DivisionByZero,
    InvalidOperand(String),
}

impl std::error::Error for RuntimeErrorKind {}

impl std::fmt::Display for RuntimeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeErrorKind::DivisionByZero => write!(f, "Division by zero error."),
            RuntimeErrorKind::InvalidOperand(msg) => write!(f, "{}", msg),
        }
    }
}
