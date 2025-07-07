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
                Stmt::Expr(_) => match result {
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                    Ok(atom) => {
                        println!("{}", atom);
                    }
                },
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

    fn visit_binary(
        &mut self,
        left: &Expr<'a>,
        op: &Op,
        right: &Expr<'a>,
    ) -> Result<Atom<'a>, Error> {
        let left_value = left.accept(self)?;
        let right_value = right.accept(self)?;
        match op {
            Op::Plus => left_value + right_value,
            Op::Minus => left_value - right_value,
            Op::Star => left_value * right_value,
            Op::Slash => left_value / right_value,
            Op::EqualEqual => Ok(Atom::Bool(left_value == right_value)),
            Op::BangEqual => Ok(Atom::Bool(left_value != right_value)),
            Op::Less => Ok(Atom::Bool(left_value < right_value)),
            Op::LessEqual => Ok(Atom::Bool(left_value <= right_value)),
            Op::Greater => Ok(Atom::Bool(left_value > right_value)),
            Op::GreaterEqual => Ok(Atom::Bool(left_value >= right_value)),
            _ => Ok(Atom::Nil),
        }
    }

    fn visit_block(&mut self, stmts: &[Stmt<'a>]) -> Result<Atom<'a>, Error> {
        todo!()
    }

    fn visit_group(&mut self, expr: &Expr<'a>) -> Result<Atom<'a>, Error> {
        expr.accept(self)
    }

    fn visit_unary(&mut self, op: &Op, expr: &Expr<'a>) -> Result<Atom<'a>, Error> {
        let value = expr.accept(self)?;
        match op {
            Op::Minus => -value,
            Op::Bang => Ok(!value),
            _ => Ok(Atom::Nil),
        }
    }
}

impl<'a> StmtVisitor<'a, Atom<'a>> for Interpreter<'a> {
    fn visit_expr_stmt(&mut self, expr: &Expr<'a>) -> Result<Atom<'a>, Error> {
        expr.accept(self)
    }

    fn visit_item_stmt(&mut self, item: &Item<'a>) -> Result<Atom<'a>, Error> {
        unimplemented!()
    }
}

/*
#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use crate::*;

    fn setup<'a>() -> Interpreter<'a> {
        let lexer = Lexer::new("");
        let parser = Parser::with_lexer(lexer);
        Interpreter::with_parser(parser)
    }

    #[test]
    fn test_true_expr() {
        let mut visitor = setup();
        let expr = Expr::Atom(Atom::Bool(true));
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Bool(true));
    }

    #[test]
    fn test_false_expr() {
        let mut visitor = setup();
        let expr = Expr::Atom(Atom::Bool(false));
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Bool(false));
    }

    #[test]
    fn test_nil_expr() {
        let mut visitor = setup();
        let expr = Expr::Atom(Atom::Nil);
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Nil);
    }

    #[test]
    fn test_string_expr() {
        let mut visitor = setup();
        let expr = Expr::Atom(Atom::String(Cow::Owned("Hello World!".to_string())));
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::String(Cow::Owned("Hello World!".to_string())));
    }

    #[test]
    fn test_number_expr() {
        let mut visitor = setup();
        let expr = Expr::Atom(Atom::Number(42.0));
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Number(42.0));
    }

    #[test]
    fn test_group_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::Number(8.0))),
            op: Op::Star,
            right: Box::new(Expr::Group(Box::new(Expr::Binary {
                left: Box::new(Expr::Atom(Atom::Number(8.0))),
                op: Op::Plus,
                right: Box::new(Expr::Atom(Atom::Number(2.0))),
            }))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Number(80.0));
    }

    #[test]
    fn test_unary_neg_expr() {
        let mut visitor = setup();
        let expr = Expr::Unary {
            op: Op::Minus,
            expr: Box::new(Expr::Atom(Atom::Number(5.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Number(-5.0));
    }

    #[test]
    fn test_unary_not_expr() {
        let mut visitor = setup();
        let expr = Expr::Unary {
            op: Op::Bang,
            expr: Box::new(Expr::Atom(Atom::Bool(true))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Bool(false));
    }

    #[test]
    fn test_unary_group_expr() {
        let mut visitor = setup();
        let expr = Expr::Unary {
            op: Op::Minus,
            expr: Box::new(Expr::Group(Box::new(Expr::Group(Box::new(Expr::Atom(
                Atom::Number(5.0),
            )))))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Number(-5.0));
    }

    #[test]
    fn test_arithmetic_mul_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::Number(5.0))),
            op: Op::Star,
            right: Box::new(Expr::Atom(Atom::Number(3.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Number(15.0));
    }

    #[test]
    fn test_arithmetic_div_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::Number(10.0))),
            op: Op::Slash,
            right: Box::new(Expr::Atom(Atom::Number(2.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Number(5.0));
    }

    #[test]
    fn test_airthmetic_mul_div_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Group(Box::new(Expr::Binary {
                left: Box::new(Expr::Atom(Atom::Number(10.40))),
                op: Op::Star,
                right: Box::new(Expr::Atom(Atom::Number(2.0))),
            }))),
            op: Op::Slash,
            right: Box::new(Expr::Atom(Atom::Number(2.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Number(10.4));
    }

    #[test]
    fn test_arithmetic_add_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::Number(5.0))),
            op: Op::Plus,
            right: Box::new(Expr::Atom(Atom::Number(3.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Number(8.0));
    }

    #[test]
    fn test_arithmetic_sub_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::Number(5.0))),
            op: Op::Minus,
            right: Box::new(Expr::Atom(Atom::Number(3.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Number(2.0));
    }

    #[test]
    fn test_arithmetic_add_sub_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Atom(Atom::Number(23.0))),
                op: Op::Plus,
                right: Box::new(Expr::Atom(Atom::Number(28.0))),
            }),
            op: Op::Minus,
            right: Box::new(Expr::Group(Box::new(Expr::Unary {
                op: Op::Minus,
                expr: Box::new(Expr::Group(Box::new(Expr::Binary {
                    left: Box::new(Expr::Atom(Atom::Number(61.0))),
                    op: Op::Minus,
                    right: Box::new(Expr::Atom(Atom::Number(99.0))),
                }))),
            }))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Number(13.0));
    }

    #[test]
    fn test_string_concatenation_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::String(Cow::Borrowed("Hello ")))),
            op: Op::Plus,
            right: Box::new(Expr::Atom(Atom::String(Cow::Borrowed("World!")))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::String(Cow::Owned("Hello World!".to_string())));
    }

    #[test]
    fn test_relational_greater_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::Number(10.0))),
            op: Op::Greater,
            right: Box::new(Expr::Atom(Atom::Number(5.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Bool(true));
    }

    #[test]
    fn test_relational_less_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::Number(3.0))),
            op: Op::Less,
            right: Box::new(Expr::Atom(Atom::Number(7.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Bool(true));
    }

    #[test]
    fn test_relational_greater_equal_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::Number(5.0))),
            op: Op::GreaterEqual,
            right: Box::new(Expr::Atom(Atom::Number(5.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Bool(true));
    }

    #[test]
    fn test_relational_less_equal_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::Number(4.0))),
            op: Op::LessEqual,
            right: Box::new(Expr::Atom(Atom::Number(5.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Bool(true));
    }

    #[test]
    fn test_huge_relational_expr() {
        let mut visitor = setup();

        let expr = Expr::Binary {
            left: Box::new(Expr::Group(Box::new(Expr::Binary {
                left: Box::new(Expr::Atom(Atom::Number(54.0))),
                op: Op::Minus,
                right: Box::new(Expr::Atom(Atom::Number(54.0))),
            }))),
            op: Op::GreaterEqual,
            right: Box::new(Expr::Unary {
                op: Op::Minus,
                expr: Box::new(Expr::Group(Box::new(Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Atom(Atom::Number(114.0))),
                        op: Op::Slash,
                        right: Box::new(Expr::Atom(Atom::Number(57.0))),
                    }),
                    op: Op::Plus,
                    right: Box::new(Expr::Atom(Atom::Number(11.0))),
                }))),
            }),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Bool(true));
    }

    #[test]
    fn test_equality_equal_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::Number(5.0))),
            op: Op::EqualEqual,
            right: Box::new(Expr::Atom(Atom::Number(5.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Bool(true));
    }

    #[test]
    fn test_equality_not_equal_expr() {
        let mut visitor = setup();
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::Number(5.0))),
            op: Op::BangEqual,
            right: Box::new(Expr::Atom(Atom::Number(3.0))),
        };
        let result = expr.accept(&mut visitor);
        assert_eq!(result, Atom::Bool(true));
    }
}
*/
