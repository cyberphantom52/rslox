use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Selmicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftParen => write!(f, "LEFT_PAREN"),
            Self::RightParen => write!(f, "RIGHT_PAREN"),
            Self::LeftBrace => write!(f, "LEFT_BRACE"),
            Self::RightBrace => write!(f, "RIGHT_BRACE"),
            Self::Comma => write!(f, "COMMA"),
            Self::Dot => write!(f, "DOT"),
            Self::Selmicolon => write!(f, "SEMICOLON"),
            Self::Plus => write!(f, "PLUS"),
            Self::Minus => write!(f, "MINUS"),
            Self::Star => write!(f, "STAR"),
            Self::Slash => write!(f, "SLASH"),
            Self::Bang => write!(f, "BANG"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    BangEqual,
    Less,
    LessEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BangEqual => write!(f, "BANG_EQUAL"),
            Self::Less => write!(f, "LESS"),
            Self::LessEqual => write!(f, "LESS_EQUAL"),
            Self::Equal => write!(f, "EQUAL"),
            Self::EqualEqual => write!(f, "EQUAL_EQUAL"),
            Self::Greater => write!(f, "GREATER"),
            Self::GreaterEqual => write!(f, "GREATER_EQUAL"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Unary(UnaryOperator),
    Binary(BinaryOperator),
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unary(op) => write!(f, "{}", op),
            Self::Binary(op) => write!(f, "{}", op),
        }
    }
}

impl TryFrom<char> for Operator {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string().as_str())
    }
}

impl TryFrom<&str> for Operator {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "(" => Ok(Operator::Unary(UnaryOperator::LeftParen)),
            ")" => Ok(Operator::Unary(UnaryOperator::RightParen)),
            "{" => Ok(Operator::Unary(UnaryOperator::LeftBrace)),
            "}" => Ok(Operator::Unary(UnaryOperator::RightBrace)),
            "," => Ok(Operator::Unary(UnaryOperator::Comma)),
            "." => Ok(Operator::Unary(UnaryOperator::Dot)),
            ";" => Ok(Operator::Unary(UnaryOperator::Selmicolon)),
            "+" => Ok(Operator::Unary(UnaryOperator::Plus)),
            "-" => Ok(Operator::Unary(UnaryOperator::Minus)),
            "*" => Ok(Operator::Unary(UnaryOperator::Star)),
            "/" => Ok(Operator::Unary(UnaryOperator::Slash)),
            "!" => Ok(Operator::Unary(UnaryOperator::Bang)),
            "!=" => Ok(Operator::Binary(BinaryOperator::BangEqual)),
            "<" => Ok(Operator::Binary(BinaryOperator::Less)),
            "<=" => Ok(Operator::Binary(BinaryOperator::LessEqual)),
            "=" => Ok(Operator::Binary(BinaryOperator::Equal)),
            "==" => Ok(Operator::Binary(BinaryOperator::EqualEqual)),
            ">" => Ok(Operator::Binary(BinaryOperator::Greater)),
            ">=" => Ok(Operator::Binary(BinaryOperator::GreaterEqual)),
            _ => Err(Error::ParseError {
                msg: format!("Unknown operator: {}", value),
            }),
        }
    }
}
