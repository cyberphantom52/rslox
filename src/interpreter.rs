use crate::{
    Parser,
    error::Error,
    token::{Atom, Expr, Item, Op, Stmt},
    visitor::{ExprVisitor, StmtVisitor},
};

pub struct Interpreter<'a> {
    parser: Parser<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn with_parser(parser: Parser<'a>) -> Self {
        Self { parser }
    }

    pub fn interpret(&mut self) -> Result<(), Error> {
        let tree = self.parser.parse()?;
        for stmt in tree.0 {
            let result = stmt.accept(self);
            match stmt {
                Stmt::Expr(_) => println!("{}", result),
                _ => {}
            }
        }
        Ok(())
    }
}

impl<'a> ExprVisitor<'a, Atom<'a>> for Interpreter<'a> {
    fn visit_atom(&mut self, atom: &Atom<'a>) -> Atom<'a> {
        atom.clone()
    }

    fn visit_binary(&mut self, left: &Expr<'a>, op: &Op, right: &Expr<'a>) -> Atom<'a> {
        let left_value = left.accept(self);
        let right_value = right.accept(self);
        match op {
            Op::Plus => left_value + right_value,
            Op::Minus => left_value - right_value,
            Op::Star => left_value * right_value,
            Op::Slash => left_value / right_value,
            Op::EqualEqual => Atom::Bool(left_value == right_value),
            Op::BangEqual => Atom::Bool(left_value != right_value),
            Op::Less => Atom::Bool(left_value < right_value),
            Op::LessEqual => Atom::Bool(left_value <= right_value),
            Op::Greater => Atom::Bool(left_value > right_value),
            Op::GreaterEqual => Atom::Bool(left_value >= right_value),
            _ => Atom::Nil,
        }
    }

    fn visit_block(&mut self, stmts: &[Stmt<'a>]) -> Atom<'a> {
        todo!()
    }

    fn visit_group(&mut self, expr: &Expr<'a>) -> Atom<'a> {
        expr.accept(self)
    }

    fn visit_unary(&mut self, op: &Op, expr: &Expr<'a>) -> Atom<'a> {
        let value = expr.accept(self);
        match op {
            Op::Minus => -value,
            Op::Bang => !value,
            _ => Atom::Nil,
        }
    }
}

impl<'a> StmtVisitor<'a, Atom<'a>> for Interpreter<'a> {
    fn visit_expr_stmt(&mut self, expr: &Expr<'a>) -> Atom<'a> {
        expr.accept(self)
    }

    fn visit_item_stmt(&mut self, item: &Item<'a>) -> Atom<'a> {
        unimplemented!()
    }
}
