use std::borrow::Cow;

use crate::error::{Error, ParseError, ParseErrorKind, RuntimeError, RuntimeErrorKind};

use super::{BinaryOperator, Keyword, Operator, UnaryOperator};

#[derive(Debug, Clone)]
pub struct TokenTree<'a>(pub Vec<Stmt<'a>>);

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    Item(Item<'a>),
    Expr(Expr<'a>),
}

impl std::fmt::Display for Stmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Item(item) => write!(f, "{}", item),
            Stmt::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Atom(Atom<'a>),
    Binary {
        left: Box<Expr<'a>>,
        op: Op,
        right: Box<Expr<'a>>,
    },
    Unary {
        op: Op,
        expr: Box<Expr<'a>>,
    },
    Group(Box<Expr<'a>>),
    Block {
        stmts: Vec<Stmt<'a>>,
    },
}

impl std::fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Atom(atom) => write!(f, "{}", atom),
            Expr::Binary { left, op, right } => write!(f, "({} {} {})", op, left, right),
            Expr::Unary { op, expr } => write!(f, "({} {})", op, expr),
            Expr::Group(expr) => write!(f, "(group {})", expr),
            Expr::Block { stmts } => {
                write!(f, "{{")?;
                for stmt in stmts {
                    write!(f, " {}", stmt)?;
                }
                write!(f, " }}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Item<'a> {
    Struct {
        name: &'a str,
        fields: Vec<(&'a str, TokenTree<'a>)>,
    },
    Function {
        name: &'a str,
        params: Vec<(&'a str, TokenTree<'a>)>,
        body: TokenTree<'a>,
    },
}

impl std::fmt::Display for Item<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Struct { name, fields } => {
                write!(f, "struct {} {{", name)?;
                for (field_name, field_type) in fields {
                    write!(f, " {}: {},", field_name, field_type)?;
                }
                write!(f, " }}")
            }
            Item::Function { name, params, body } => {
                write!(f, "fn {}(", name)?;
                for (param_name, param_type) in params {
                    write!(f, "{}: {}, ", param_name, param_type)?;
                }
                write!(f, ") -> {} {{ {} }}", body, body)
            }
        }
    }
}

impl std::fmt::Display for TokenTree<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.0 {
            write!(f, "{}\n", stmt)?;
        }
        Ok(())
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

impl<'a> std::ops::Add for Atom<'a> {
    type Output = Result<Atom<'a>, Error>;

    fn add(self, other: Atom<'_>) -> Self::Output {
        match (self, other) {
            (Atom::String(s1), Atom::String(s2)) => {
                Ok(Atom::String(Cow::Owned(format!("{}{}", s1, s2))))
            }
            (Atom::Number(n1), Atom::Number(n2)) => Ok(Atom::Number(n1 + n2)),
            _ => Err(Error::RuntimeError(RuntimeError::new(
                RuntimeErrorKind::InvalidOperand(
                    "Operands must be two numbers or two strings.".to_string(),
                ),
            ))),
        }
    }
}

impl<'a> std::ops::Sub for Atom<'a> {
    type Output = Result<Atom<'a>, Error>;

    fn sub(self, other: Atom<'_>) -> Self::Output {
        match (self, other) {
            (Atom::Number(n1), Atom::Number(n2)) => Ok(Atom::Number(n1 - n2)),
            _ => Err(Error::RuntimeError(RuntimeError::new(
                RuntimeErrorKind::InvalidOperand("Operands must be numbers".to_string()),
            ))),
        }
    }
}

impl<'a> std::ops::Mul for Atom<'a> {
    type Output = Result<Atom<'a>, Error>;

    fn mul(self, other: Atom<'_>) -> Self::Output {
        match (self, other) {
            (Atom::Number(n1), Atom::Number(n2)) => Ok(Atom::Number(n1 * n2)),
            _ => Err(Error::RuntimeError(RuntimeError::new(
                RuntimeErrorKind::InvalidOperand("Operands must be numbers".to_string()),
            ))),
        }
    }
}

impl<'a> std::ops::Div for Atom<'a> {
    type Output = Result<Atom<'a>, Error>;

    fn div(self, other: Atom<'_>) -> Self::Output {
        match (self, other) {
            (Atom::Number(n1), Atom::Number(n2)) => Ok(if n2 == 0.0 {
                Atom::Nil
            } else {
                Atom::Number(n1 / n2)
            }),
            _ => Err(Error::RuntimeError(RuntimeError::new(
                RuntimeErrorKind::InvalidOperand("Operands must be numbers".to_string()),
            ))),
        }
    }
}

impl<'a> std::ops::Neg for Atom<'a> {
    type Output = Result<Atom<'a>, Error>;

    fn neg(self) -> Self::Output {
        match self {
            Atom::Number(n) => Ok(Atom::Number(-n)),
            _ => Err(Error::RuntimeError(RuntimeError::new(
                RuntimeErrorKind::InvalidOperand(format!("Operand must be a number.")),
            ))),
        }
    }
}

impl<'a> std::ops::Not for Atom<'a> {
    type Output = Atom<'a>;

    fn not(self) -> Atom<'a> {
        match self {
            Atom::Bool(b) => Atom::Bool(!b),
            _ => Atom::Nil,
        }
    }
}

impl<'a> std::cmp::PartialEq for Atom<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Atom::String(s1), Atom::String(s2)) => s1 == s2,
            (Atom::Number(n1), Atom::Number(n2)) => n1 == n2,
            (Atom::Nil, Atom::Nil) => true,
            (Atom::Bool(b1), Atom::Bool(b2)) => b1 == b2,
            (Atom::Ident(i1), Atom::Ident(i2)) => i1 == i2,
            _ => false,
        }
    }
}

impl<'a> std::cmp::PartialOrd for Atom<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Atom::Number(n1), Atom::Number(n2)) => n1.partial_cmp(n2),
            (Atom::String(s1), Atom::String(s2)) => s1.partial_cmp(s2),
            _ => None,
        }
    }
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
    pub fn prefix_binding_power(&self) -> Option<((), u8)> {
        match self {
            Op::Print | Op::Return => Some(((), 1)),
            Op::Bang | Op::Plus | Op::Minus => Some(((), 11)),
            _ => None,
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
            op => Err(Error::ParseError(ParseError::new(
                ParseErrorKind::UnsupportedOperator(Operator::Unary(op)),
            ))),
        }
    }
}

impl TryFrom<Keyword> for Op {
    type Error = Error;

    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            Keyword::Print => Ok(Op::Print),
            Keyword::Return => Ok(Op::Return),
            _ => Err(Error::ParseError(ParseError::new(
                ParseErrorKind::UnsupportedKeyword(value),
            ))),
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
