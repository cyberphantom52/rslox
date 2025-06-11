use std::borrow::Cow;

use crate::error::Error;

use super::{BinaryOperator, Keyword, Operator, UnaryOperator};

#[derive(Debug, Clone)]
pub enum TokenTree<'a> {
    Atom(Atom<'a>),
    Cons(Op, Vec<TokenTree<'a>>),
}

impl std::fmt::Display for TokenTree<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenTree::Atom(atom) => write!(f, "{}", atom),
            TokenTree::Cons(op, children) => {
                write!(f, "({}", op)?;
                for s in children {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Atom<'a> {
    String(Cow<'a, str>),
    Number(f64),
    Nil,
    Bool(bool),
    Ident(&'a str),
    Super,
    This,
}

impl std::fmt::Display for Atom<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(s) => write!(f, "{}", s),
            Atom::Number(n) => {
                if n.fract() == 0f64 {
                    write!(f, "{}.0", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Atom::Nil => write!(f, "nil"),
            Atom::Bool(b) => write!(f, "{}", b),
            Atom::Ident(i) => write!(f, "{}", i),
            Atom::Super => write!(f, "super"),
            Atom::This => write!(f, "this"),
        }
    }
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

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Call => write!(f, "call"),
            Op::Dot => write!(f, "."),
            Op::Group => write!(f, "group"),
            Op::Class => write!(f, "class"),
            Op::And => write!(f, "and"),
            Op::Or => write!(f, "or"),
            Op::Var => write!(f, "var"),
            Op::Print => write!(f, "print"),
            Op::While => write!(f, "while"),
            Op::For => write!(f, "for"),
            Op::If => write!(f, "if"),
            Op::Return => write!(f, "return"),
            Op::Plus => write!(f, "+"),
            Op::Minus => write!(f, "-"),
            Op::Star => write!(f, "*"),
            Op::Slash => write!(f, "/"),
            Op::Bang => write!(f, "!"),
            Op::BangEqual => write!(f, "!="),
            Op::Less => write!(f, "<"),
            Op::LessEqual => write!(f, "<="),
            Op::Equal => write!(f, "="),
            Op::EqualEqual => write!(f, "=="),
            Op::Greater => write!(f, ">"),
            Op::GreaterEqual => write!(f, ">="),
        }
    }
}

impl Op {
    pub fn prefix_binding_power(&self) -> ((), u8) {
        match self {
            Op::Print | Op::Return => ((), 1),
            Op::Bang | Op::Plus | Op::Minus => ((), 11),
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
            Op::BangEqual
            | Op::EqualEqual
            | Op::Less
            | Op::LessEqual
            | Op::Greater
            | Op::GreaterEqual => (5, 6),
            Op::Plus | Op::Minus => (7, 8),
            Op::Star | Op::Slash => (9, 10),
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

impl TryFrom<Keyword> for Op {
    type Error = Error;

    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            Keyword::Print => Ok(Op::Print),
            Keyword::Return => Ok(Op::Return),
            _ => Err(Error::ParseError {
                msg: format!("Unsupported keyword: {:?}", value),
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
