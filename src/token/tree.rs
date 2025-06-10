use crate::error::Error;

use super::{BinaryOperator, Operator, UnaryOperator};

#[derive(Debug, Clone)]
pub enum TokenTree<'a> {
    Atom(Atom<'a>),
    Cons(Op, Vec<TokenTree<'a>>),
}

#[derive(Debug, Clone)]
pub enum Atom<'a> {
    String(&'a str),
    Number(f64),
    Nil,
    Bool(bool),
    Ident(&'a str),
    Super,
    This,
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Call,
    Dot,
    Group,

    Class,
    And,
    Or,
    Var,
    Print,

    While,
    For,
    If,
    Return,

    Plus,
    Minus,
    Star,
    Slash,

    Bang,
    BangEqual,
    Less,
    LessEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
}

impl Op {
    pub fn prefix_binding_power(&self) -> ((), u8) {
        match self {
            Op::Bang | Op::Plus | Op::Minus => ((), 9),
            _ => panic!("bad op: {:?}", self),
        }
    }

    pub fn postfix_binding_power(&self) -> Option<(u8, ())> {
        let res = match self {
            Op::Bang => (11, ()),
            // '[' => (11, ()),
            _ => return None,
        };
        Some(res)
    }

    pub fn infix_binding_power(&self) -> Option<(u8, u8)> {
        let res = match self {
            Op::Equal => (2, 1),
            // '?' => (4, 3),
            Op::Plus | Op::Minus => (5, 6),
            Op::Star | Op::Slash => (7, 8),
            Op::Dot => (14, 13),
            _ => return None,
        };
        Some(res)
    }
}

impl TryFrom<UnaryOperator> for Op {
    type Error = Error;

    fn try_from(value: UnaryOperator) -> Result<Self, Self::Error> {
        match value {
            UnaryOperator::Dot => Ok(Op::Dot),
            UnaryOperator::Minus => Ok(Op::Minus),
            UnaryOperator::Plus => Ok(Op::Plus),
            UnaryOperator::Star => Ok(Op::Star),
            UnaryOperator::Slash => Ok(Op::Slash),
            UnaryOperator::Bang => Ok(Op::Bang),
            op => Err(Error::ParseError {
                msg: format!("Unsupported unary operator: {:?}", op),
            }),
        }
    }
}

impl TryFrom<BinaryOperator> for Op {
    type Error = Error;

    fn try_from(value: BinaryOperator) -> Result<Self, Self::Error> {
        match value {
            BinaryOperator::Equal => Ok(Op::Equal),
            BinaryOperator::BangEqual => Ok(Op::BangEqual),
            BinaryOperator::EqualEqual => Ok(Op::EqualEqual),
            BinaryOperator::Less => Ok(Op::Less),
            BinaryOperator::LessEqual => Ok(Op::LessEqual),
            BinaryOperator::Greater => Ok(Op::Greater),
            BinaryOperator::GreaterEqual => Ok(Op::GreaterEqual),
        }
    }
}

impl TryFrom<Operator> for Op {
    type Error = Error;

    fn try_from(value: Operator) -> Result<Self, Self::Error> {
        match value {
            Operator::Unary(op) => op.try_into(),
            Operator::Binary(op) => op.try_into(),
        }
    }
}
