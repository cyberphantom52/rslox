use crate::token::{Atom, Expr, Item, Op, Stmt};

pub trait ExprVisitor<'a, T> {
    fn visit_atom(&mut self, atom: &Atom<'a>) -> T;
    fn visit_binary(&mut self, left: &Expr<'a>, op: &Op, right: &Expr<'a>) -> T;
    fn visit_unary(&mut self, op: &Op, expr: &Expr<'a>) -> T;
    fn visit_group(&mut self, expr: &Expr<'a>) -> T;
    fn visit_block(&mut self, stmts: &[Stmt<'a>]) -> T;
}

pub trait StmtVisitor<'a, T> {
    fn visit_expr_stmt(&mut self, expr: &Expr<'a>) -> T;
    fn visit_item_stmt(&mut self, item: &Item<'a>) -> T;
}

impl<'a> Expr<'a> {
    pub fn accept<T, V: ExprVisitor<'a, T>>(&self, visitor: &mut V) -> T {
        match self {
            Expr::Atom(atom) => visitor.visit_atom(atom),
            Expr::Binary { left, op, right } => visitor.visit_binary(left, op, right),
            Expr::Unary { op, expr } => visitor.visit_unary(op, expr),
            Expr::Group(expr) => visitor.visit_group(expr),
            Expr::Block { stmts } => visitor.visit_block(stmts),
        }
    }
}

impl<'a> Stmt<'a> {
    pub fn accept<T, V: StmtVisitor<'a, T>>(&self, visitor: &mut V) -> T {
        match self {
            Stmt::Expr(expr) => visitor.visit_expr_stmt(expr),
            Stmt::Item(item) => visitor.visit_item_stmt(item),
        }
    }
}
