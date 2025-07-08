use crate::{
    ParseResult, Parser,
    error::{Error, RuntimeError},
    token::{Atom, AtomKind, Expr, Item, Op, Stmt, merge_span},
    visitor::{ExprVisitor, StmtVisitor},
};

pub struct Interpreter<'a> {
    parser: Parser<'a>,
}

impl<'a> From<Parser<'a>> for Interpreter<'a> {
    fn from(parser: Parser<'a>) -> Self {
        Self { parser }
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(source: &'a str) -> Self {
        let parser = Parser::new(source);
        Self { parser }
    }

    pub fn interpret(&mut self) -> Result<(), Error> {
        let ParseResult { tree, .. } = self.parser.parse();

        for stmt in tree.0 {
            let result = stmt.accept(self);
            match stmt {
                Stmt::Expr(_) => match result {
                    Err(e) => {
                        eprintln!("{:?}", miette::Report::new(e));
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
            Op::Plus => (left_value + right_value).map_err(|kind| {
                Error::RuntimeError(RuntimeError::new(
                    self.parser.lexer().source_code().to_string(),
                    kind,
                    merge_span(left.span(), right.span()),
                ))
            }),
            Op::Minus => (left_value - right_value).map_err(|kind| {
                Error::RuntimeError(RuntimeError::new(
                    self.parser.lexer().source_code().to_string(),
                    kind,
                    merge_span(left.span(), right.span()),
                ))
            }),
            Op::Star => (left_value * right_value).map_err(|kind| {
                Error::RuntimeError(RuntimeError::new(
                    self.parser.lexer().source_code().to_string(),
                    kind,
                    merge_span(left.span(), right.span()),
                ))
            }),
            Op::Slash => (left_value / right_value).map_err(|kind| {
                Error::RuntimeError(RuntimeError::new(
                    self.parser.lexer().source_code().to_string(),
                    kind,
                    merge_span(left.span(), right.span()),
                ))
            }),
            Op::EqualEqual => Ok(Atom::new(
                AtomKind::Bool(left_value == right_value),
                merge_span(left.span(), right.span()),
            )),
            Op::BangEqual => Ok(Atom::new(
                AtomKind::Bool(left_value != right_value),
                merge_span(left.span(), right.span()),
            )),
            Op::Less => Ok(Atom::new(
                AtomKind::Bool(left_value < right_value),
                merge_span(left.span(), right.span()),
            )),
            Op::LessEqual => Ok(Atom::new(
                AtomKind::Bool(left_value <= right_value),
                merge_span(left.span(), right.span()),
            )),
            Op::Greater => Ok(Atom::new(
                AtomKind::Bool(left_value > right_value),
                merge_span(left.span(), right.span()),
            )),
            Op::GreaterEqual => Ok(Atom::new(
                AtomKind::Bool(left_value >= right_value),
                merge_span(left.span(), right.span()),
            )),
            _ => Ok(Atom::new(
                AtomKind::Nil,
                merge_span(left.span(), right.span()),
            )),
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
            Op::Minus => (-value).map_err(|kind| {
                Error::RuntimeError(RuntimeError::new(
                    self.parser.lexer().source_code().to_string(),
                    kind,
                    expr.span(),
                ))
            }),
            Op::Bang => Ok(!value),
            _ => Ok(Atom::new(AtomKind::Nil, expr.span())),
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

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use miette::SourceSpan;

    use super::*;

    #[test]
    fn test_true_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Atom(Atom::new(
            AtomKind::Bool(true),
            SourceSpan::new(0.into(), 0),
        ));
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Bool(true));
    }

    #[test]
    fn test_false_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Atom(Atom::new(
            AtomKind::Bool(false),
            SourceSpan::new(0.into(), 0),
        ));
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Bool(false));
    }

    #[test]
    fn test_nil_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Atom(Atom::new(AtomKind::Nil, SourceSpan::new(0.into(), 0)));
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Nil);
    }

    #[test]
    fn test_string_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Atom(Atom::new(
            AtomKind::String(Cow::Owned("Hello World!".to_string())),
            SourceSpan::new(0.into(), 0),
        ));
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(
            *result.kind(),
            AtomKind::String(Cow::Owned("Hello World!".to_string()))
        );
    }

    #[test]
    fn test_number_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Atom(Atom::new(
            AtomKind::Number(42.0),
            SourceSpan::new(0.into(), 0),
        ));
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Number(42.0));
    }

    #[test]
    fn test_group_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(8.0),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::Star,
            right: Box::new(Expr::Group(Box::new(Expr::Binary {
                left: Box::new(Expr::Atom(Atom::new(
                    AtomKind::Number(8.0),
                    SourceSpan::new(0.into(), 0),
                ))),
                op: Op::Plus,
                right: Box::new(Expr::Atom(Atom::new(
                    AtomKind::Number(2.0),
                    SourceSpan::new(0.into(), 0),
                ))),
            }))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Number(80.0));
    }

    #[test]
    fn test_unary_neg_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Unary {
            op: Op::Minus,
            expr: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(5.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Number(-5.0));
    }

    #[test]
    fn test_unary_not_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Unary {
            op: Op::Bang,
            expr: Box::new(Expr::Atom(Atom::new(
                AtomKind::Bool(true),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Bool(false));
    }

    #[test]
    fn test_unary_group_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Unary {
            op: Op::Minus,
            expr: Box::new(Expr::Group(Box::new(Expr::Group(Box::new(Expr::Atom(
                Atom::new(AtomKind::Number(5.0), SourceSpan::new(0.into(), 0)),
            )))))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Number(-5.0));
    }

    #[test]
    fn test_arithmetic_mul_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(5.0),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::Star,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(3.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Number(15.0));
    }

    #[test]
    fn test_arithmetic_div_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(10.0),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::Slash,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(2.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Number(5.0));
    }

    #[test]
    fn test_airthmetic_mul_div_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Group(Box::new(Expr::Binary {
                left: Box::new(Expr::Atom(Atom::new(
                    AtomKind::Number(10.40),
                    SourceSpan::new(0.into(), 0),
                ))),
                op: Op::Star,
                right: Box::new(Expr::Atom(Atom::new(
                    AtomKind::Number(2.0),
                    SourceSpan::new(0.into(), 0),
                ))),
            }))),
            op: Op::Slash,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(2.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Number(10.4));
    }

    #[test]
    fn test_arithmetic_add_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(5.0),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::Plus,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(3.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Number(8.0));
    }

    #[test]
    fn test_arithmetic_sub_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(5.0),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::Minus,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(3.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Number(2.0));
    }

    #[test]
    fn test_arithmetic_add_sub_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::Atom(Atom::new(
                    AtomKind::Number(23.0),
                    SourceSpan::new(0.into(), 0),
                ))),
                op: Op::Plus,
                right: Box::new(Expr::Atom(Atom::new(
                    AtomKind::Number(28.0),
                    SourceSpan::new(0.into(), 0),
                ))),
            }),
            op: Op::Minus,
            right: Box::new(Expr::Group(Box::new(Expr::Unary {
                op: Op::Minus,
                expr: Box::new(Expr::Group(Box::new(Expr::Binary {
                    left: Box::new(Expr::Atom(Atom::new(
                        AtomKind::Number(61.0),
                        SourceSpan::new(0.into(), 0),
                    ))),
                    op: Op::Minus,
                    right: Box::new(Expr::Atom(Atom::new(
                        AtomKind::Number(99.0),
                        SourceSpan::new(0.into(), 0),
                    ))),
                }))),
            }))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Number(13.0));
    }

    #[test]
    fn test_string_concatenation_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::String(Cow::Borrowed("Hello ")),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::Plus,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::String(Cow::Borrowed("World!")),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(
            *result.kind(),
            AtomKind::String(Cow::Owned("Hello World!".to_string()))
        );
    }

    #[test]
    fn test_relational_greater_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(10.0),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::Greater,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(5.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Bool(true));
    }

    #[test]
    fn test_relational_less_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(3.0),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::Less,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(7.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Bool(true));
    }

    #[test]
    fn test_relational_greater_equal_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(5.0),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::GreaterEqual,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(5.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Bool(true));
    }

    #[test]
    fn test_relational_less_equal_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(4.0),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::LessEqual,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(5.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Bool(true));
    }

    #[test]
    fn test_huge_relational_expr() {
        let mut visitor = Interpreter::new("");

        let expr = Expr::Binary {
            left: Box::new(Expr::Group(Box::new(Expr::Binary {
                left: Box::new(Expr::Atom(Atom::new(
                    AtomKind::Number(54.0),
                    SourceSpan::new(0.into(), 0),
                ))),
                op: Op::Minus,
                right: Box::new(Expr::Atom(Atom::new(
                    AtomKind::Number(54.0),
                    SourceSpan::new(0.into(), 0),
                ))),
            }))),
            op: Op::GreaterEqual,
            right: Box::new(Expr::Unary {
                op: Op::Minus,
                expr: Box::new(Expr::Group(Box::new(Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Atom(Atom::new(
                            AtomKind::Number(114.0),
                            SourceSpan::new(0.into(), 0),
                        ))),
                        op: Op::Slash,
                        right: Box::new(Expr::Atom(Atom::new(
                            AtomKind::Number(57.0),
                            SourceSpan::new(0.into(), 0),
                        ))),
                    }),
                    op: Op::Plus,
                    right: Box::new(Expr::Atom(Atom::new(
                        AtomKind::Number(11.0),
                        SourceSpan::new(0.into(), 0),
                    ))),
                }))),
            }),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Bool(true));
    }

    #[test]
    fn test_equality_equal_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(5.0),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::EqualEqual,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(5.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Bool(true));
    }

    #[test]
    fn test_equality_not_equal_expr() {
        let mut visitor = Interpreter::new("");
        let expr = Expr::Binary {
            left: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(5.0),
                SourceSpan::new(0.into(), 0),
            ))),
            op: Op::BangEqual,
            right: Box::new(Expr::Atom(Atom::new(
                AtomKind::Number(3.0),
                SourceSpan::new(0.into(), 0),
            ))),
        };
        let result = expr.accept(&mut visitor).unwrap();
        assert_eq!(*result.kind(), AtomKind::Bool(true));
    }
}
