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

#[derive(Debug, PartialEq)]
pub enum Operator {
    Unary(UnaryOperator),
    Binary(BinaryOperator),
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
            _ => Err(Error::UnexpectedToken(value.to_string())),
        }
    }
}
