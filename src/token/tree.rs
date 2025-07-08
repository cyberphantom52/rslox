use super::{BinaryOperator, Keyword, Operator, UnaryOperator};
use crate::error::{ParseErrorKind, RuntimeErrorKind};
use miette::SourceSpan;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct TokenTree<'a>(pub Vec<Stmt<'a>>);

#[derive(Debug, Clone)]
pub enum Stmt<'a> {
    Item(Item<'a>),
    Expr(Expr<'a>),
}

impl Stmt<'_> {
    pub fn span(&self) -> SourceSpan {
        match self {
            Stmt::Item(_) => todo!(),
            Stmt::Expr(expr) => expr.span(),
        }
    }
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

impl Expr<'_> {
    pub fn span(&self) -> SourceSpan {
        match self {
            Expr::Atom(atom) => atom.span(),
            Expr::Binary { left, right, .. } => merge_span(left.span(), right.span()),
            Expr::Unary { expr, .. } => expr.span(),
            Expr::Group(expr) => expr.span(),
            Expr::Block { stmts } => {
                todo!()
            }
        }
    }
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
pub struct Atom<'a> {
    kind: AtomKind<'a>,
    span: SourceSpan,
}

impl<'a> Atom<'a> {
    pub fn new(kind: AtomKind<'a>, span: SourceSpan) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> &AtomKind<'a> {
        &self.kind
    }

    pub fn span(&self) -> SourceSpan {
        self.span
    }
}

#[derive(Debug, Clone)]
pub enum AtomKind<'a> {
    String(Cow<'a, str>),
    Number(f64),
    Nil,
    Bool(bool),
    Ident(&'a str),
    Super,
    This,
}

pub fn merge_span(s1: SourceSpan, s2: SourceSpan) -> SourceSpan {
    let start = usize::min(s1.offset(), s2.offset());
    let s1_end = s1.offset() + s1.len();
    let s2_end = s2.offset() + s2.len();
    let length = usize::max(s1_end, s2_end) - start;

    SourceSpan::new(start.into(), length)
}

impl<'a> std::ops::Add for Atom<'a> {
    type Output = Result<Atom<'a>, RuntimeErrorKind>;

    fn add(self, other: Atom<'_>) -> Self::Output {
        let kind = match (self.kind(), other.kind()) {
            (AtomKind::String(s1), AtomKind::String(s2)) => {
                AtomKind::String(Cow::Owned(format!("{}{}", s1, s2)))
            }
            (AtomKind::Number(n1), AtomKind::Number(n2)) => AtomKind::Number(n1 + n2),
            _ => {
                return Err(RuntimeErrorKind::InvalidOperand(
                    "Operands must be two numbers or two strings.".to_string(),
                ));
            }
        };
        let span = merge_span(self.span(), other.span());
        Ok(Atom::new(kind, span))
    }
}

impl<'a> std::ops::Sub for Atom<'a> {
    type Output = Result<Atom<'a>, RuntimeErrorKind>;

    fn sub(self, other: Atom<'_>) -> Self::Output {
        match (self.kind(), other.kind()) {
            (AtomKind::Number(n1), AtomKind::Number(n2)) => Ok(Atom::new(
                AtomKind::Number(n1 - n2),
                merge_span(self.span(), other.span()),
            )),
            _ => Err(RuntimeErrorKind::InvalidOperand(
                "Operands must be numbers".to_string(),
            )),
        }
    }
}

impl<'a> std::ops::Mul for Atom<'a> {
    type Output = Result<Atom<'a>, RuntimeErrorKind>;

    fn mul(self, other: Atom<'_>) -> Self::Output {
        match (self.kind(), other.kind()) {
            (AtomKind::Number(n1), AtomKind::Number(n2)) => Ok(Atom::new(
                AtomKind::Number(n1 * n2),
                merge_span(self.span(), other.span()),
            )),
            _ => Err(RuntimeErrorKind::InvalidOperand(
                "Operands must be numbers".to_string(),
            )),
        }
    }
}

impl<'a> std::ops::Div for Atom<'a> {
    type Output = Result<Atom<'a>, RuntimeErrorKind>;

    fn div(self, other: Atom<'_>) -> Self::Output {
        match (self.kind(), other.kind()) {
            (AtomKind::Number(n1), AtomKind::Number(n2)) => {
                if *n2 == 0.0 {
                    Err(RuntimeErrorKind::DivisionByZero)
                } else {
                    Ok(Atom::new(
                        AtomKind::Number(n1 / n2),
                        merge_span(self.span(), other.span()),
                    ))
                }
            }
            _ => Err(RuntimeErrorKind::InvalidOperand(
                "Operands must be numbers".to_string(),
            )),
        }
    }
}

impl<'a> std::ops::Neg for Atom<'a> {
    type Output = Result<Atom<'a>, RuntimeErrorKind>;

    fn neg(self) -> Self::Output {
        match self.kind() {
            AtomKind::Number(n) => Ok(Atom::new(AtomKind::Number(-n), self.span())),
            _ => Err(RuntimeErrorKind::InvalidOperand(format!(
                "Operand must be a number."
            ))),
        }
    }
}

impl<'a> std::ops::Not for Atom<'a> {
    type Output = Atom<'a>;

    fn not(self) -> Atom<'a> {
        match self.kind() {
            AtomKind::Bool(b) => Atom::new(AtomKind::Bool(!b), self.span()),
            _ => Atom::new(AtomKind::Nil, self.span()),
        }
    }
}

impl<'a> std::cmp::PartialEq for Atom<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self.kind(), other.kind()) {
            (AtomKind::String(s1), AtomKind::String(s2)) => s1 == s2,
            (AtomKind::Number(n1), AtomKind::Number(n2)) => n1 == n2,
            (AtomKind::Nil, AtomKind::Nil) => true,
            (AtomKind::Bool(b1), AtomKind::Bool(b2)) => b1 == b2,
            (AtomKind::Ident(i1), AtomKind::Ident(i2)) => i1 == i2,
            _ => false,
        }
    }
}

impl<'a> std::cmp::PartialOrd for Atom<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.kind(), other.kind()) {
            (AtomKind::Number(n1), AtomKind::Number(n2)) => n1.partial_cmp(n2),
            (AtomKind::String(s1), AtomKind::String(s2)) => s1.partial_cmp(s2),
            _ => None,
        }
    }
}

impl std::fmt::Display for Atom<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind() {
            AtomKind::String(s) => write!(f, "{}", s),
            AtomKind::Number(n) => {
                if n.fract() == 0f64 {
                    write!(f, "{}.0", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            AtomKind::Nil => write!(f, "nil"),
            AtomKind::Bool(b) => write!(f, "{}", b),
            AtomKind::Ident(i) => write!(f, "{}", i),
            AtomKind::Super => write!(f, "super"),
            AtomKind::This => write!(f, "this"),
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
    type Error = ParseErrorKind;

    fn try_from(value: UnaryOperator) -> Result<Self, Self::Error> {
        match value {
            UnaryOperator::Dot => Ok(Op::Dot),
            UnaryOperator::Minus => Ok(Op::Minus),
            UnaryOperator::Plus => Ok(Op::Plus),
            UnaryOperator::Star => Ok(Op::Star),
            UnaryOperator::Slash => Ok(Op::Slash),
            UnaryOperator::Bang => Ok(Op::Bang),
            op => Err(ParseErrorKind::UnsupportedOperator(Operator::Unary(op))),
        }
    }
}

impl TryFrom<Keyword> for Op {
    type Error = ParseErrorKind;

    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            Keyword::Print => Ok(Op::Print),
            Keyword::Return => Ok(Op::Return),
            _ => Err(ParseErrorKind::UnsupportedKeyword(value)),
        }
    }
}

impl TryFrom<BinaryOperator> for Op {
    type Error = ParseErrorKind;

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
    type Error = ParseErrorKind;

    fn try_from(value: Operator) -> Result<Self, Self::Error> {
        match value {
            Operator::Unary(op) => op.try_into(),
            Operator::Binary(op) => op.try_into(),
        }
    }
}
