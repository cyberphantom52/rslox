use crate::{
    error::{Error, ParseError, ParseErrorKind},
    lexer::Lexer,
    token::{
        Atom, Expr, Keyword, Literal, Op, Operator, Stmt, Token, TokenTree, TokenType,
        UnaryOperator,
    },
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn with_lexer(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<TokenTree<'a>, Error> {
        let mut stmts = Vec::new();
        while let Some(_) = self.lexer.peek() {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }
        Ok(TokenTree(stmts))
    }

    // TODO: Implement parsing for items (functions, classes, etc.)
    fn parse_stmt(&mut self) -> Result<Stmt<'a>, Error> {
        let expr = self.parse_expr(0)?;
        self.lexer.expect(TokenType::Operator(Operator::Unary(
            UnaryOperator::Selmicolon,
        )))?;
        Ok(Stmt::Expr(expr))
    }

    fn parse_expr(&mut self, min_bp: u8) -> Result<Expr<'a>, Error> {
        let lhs = match self.lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(e)) => return Err(e),
            None => {
                return Err(Error::ParseError(ParseError::with_line(
                    ParseErrorKind::InvalidExpression(String::new()),
                    self.lexer.line(),
                )));
            }
        };

        let mut lhs = match lhs.ty() {
            TokenType::Literal(lit) => match lit {
                Literal::String => Expr::Atom(Atom::String(Token::unescape(lhs.lexeme()))),
                Literal::Identifier => Expr::Atom(Atom::Ident(lhs.lexeme())),
                Literal::Number(n) => Expr::Atom(Atom::Number(n)),
            },
            TokenType::Keyword(kw) => match kw {
                Keyword::True => Expr::Atom(Atom::Bool(true)),
                Keyword::False => Expr::Atom(Atom::Bool(false)),
                Keyword::Nil => Expr::Atom(Atom::Nil),
                Keyword::This => Expr::Atom(Atom::This),
                Keyword::Super => Expr::Atom(Atom::Super),
                Keyword::Print | Keyword::Return => {
                    // Safe to unwrap as we checked the token type
                    let op: Op = kw.try_into()?;
                    let ((), r_bp) = op.prefix_binding_power().unwrap();
                    let rhs = self.parse_expr(r_bp)?;
                    Expr::Unary {
                        op,
                        expr: Box::new(rhs),
                    }
                }
                _ => {
                    return Err(Error::ParseError(ParseError::new(
                        ParseErrorKind::UnexpectedKeyword(kw),
                    )));
                }
            },
            TokenType::Operator(Operator::Unary(op)) => match op {
                UnaryOperator::LeftParen => {
                    let lhs = self.parse_expr(0)?;

                    self.lexer.expect(TokenType::Operator(Operator::Unary(
                        UnaryOperator::RightParen,
                    )))?;

                    Expr::Group(Box::new(lhs))
                }
                UnaryOperator::Bang | UnaryOperator::Minus | UnaryOperator::Plus => {
                    // Safe to unwrap as we checked the token type
                    let op: Op = op.try_into()?;
                    let ((), r_bp) = op.prefix_binding_power().unwrap();
                    let rhs = self.parse_expr(r_bp)?;
                    Expr::Unary {
                        op,
                        expr: Box::new(rhs),
                    }
                }
                _ => {
                    return Err(Error::ParseError(ParseError::with_line(
                        ParseErrorKind::InvalidExpression(lhs.lexeme().to_string()),
                        self.lexer.line(),
                    )));
                }
            },
            _ => {
                return Err(Error::ParseError(ParseError::new(
                    ParseErrorKind::UnexpectedToken(lhs.ty(), lhs.lexeme().to_string()),
                )));
            }
        };

        loop {
            let op: Op = match self.lexer.peek() {
                Some(token) => {
                    let token = token?;
                    match token.ty() {
                        TokenType::Operator(Operator::Unary(
                            UnaryOperator::RightParen | UnaryOperator::Selmicolon,
                        )) => break,
                        TokenType::Operator(op) => op.try_into()?,
                        ty => {
                            return Err(Error::ParseError(ParseError::new(
                                ParseErrorKind::UnexpectedToken(ty, token.lexeme().to_string()),
                            )));
                        }
                    }
                }
                _ => break,
            };

            if let Some((l_bp, ())) = op.postfix_binding_power() {
                if l_bp < min_bp {
                    break;
                }
                self.lexer.next();

                lhs = Expr::Unary {
                    op,
                    expr: Box::new(lhs),
                };
                continue;
            }

            if let Some((l_bp, r_bp)) = op.infix_binding_power() {
                if l_bp < min_bp {
                    break;
                }
                self.lexer.next();

                let rhs = self.parse_expr(r_bp)?;

                lhs = Expr::Binary {
                    left: Box::new(lhs),
                    op,
                    right: Box::new(rhs),
                };
                continue;
            }

            break;
        }

        Ok(lhs)
    }
}
