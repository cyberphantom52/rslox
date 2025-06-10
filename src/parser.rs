use crate::{
    error::Error,
    lexer::Lexer,
    token::{Atom, Keyword, Literal, Op, Operator, Token, TokenTree, TokenType, UnaryOperator},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn with_lexer(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<TokenTree<'a>, Error> {
        self.parse_expr(0)
    }

    fn parse_expr(&mut self, min_bp: u8) -> Result<TokenTree<'a>, Error> {
        let lhs = match self.lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(e)) => return Err(e),
            None => return Ok(TokenTree::Atom(Atom::Nil)),
        };

        let mut lhs = match lhs.ty() {
            TokenType::Operator(Operator::Unary(op)) => match op {
                UnaryOperator::LeftParen => {
                    let lhs = self.parse_expr(0)?;

                    self.lexer.expect(TokenType::Operator(Operator::Unary(
                        UnaryOperator::RightParen,
                    )))?;

                    TokenTree::Cons(Op::Group, vec![lhs])
                }
                UnaryOperator::Bang | UnaryOperator::Minus | UnaryOperator::Plus => {
                    let op: Op = op.try_into().unwrap();
                    let ((), r_bp) = op.prefix_binding_power();
                    let rhs = self.parse_expr(r_bp)?;
                    TokenTree::Cons(op, vec![rhs])
                }
                _ => {
                    return Err(Error::ParseError {
                        msg: format!("Unexpected {:?}", lhs),
                    });
                }
            },
            TokenType::Literal(lit) => match lit {
                Literal::String => TokenTree::Atom(Atom::String(Token::unescape(lhs.lexeme()))),
                Literal::Identifier => TokenTree::Atom(Atom::Ident(lhs.lexeme())),
                Literal::Number(n) => TokenTree::Atom(Atom::Number(n)),
            },
            TokenType::Keyword(kw) => match kw {
                Keyword::True => TokenTree::Atom(Atom::Bool(true)),
                Keyword::False => TokenTree::Atom(Atom::Bool(false)),
                Keyword::Nil => TokenTree::Atom(Atom::Nil),
                Keyword::This => TokenTree::Atom(Atom::This),
                Keyword::Super => TokenTree::Atom(Atom::Super),
                _ => {
                    return Err(Error::ParseError {
                        msg: format!("Unexpected keyword {:?}", kw),
                    });
                }
            },
            _ => {
                return Err(Error::ParseError {
                    msg: format!("Unexpected token {:?}", lhs),
                });
            }
        };

        loop {
            let op: Op = match self.lexer.peek() {
                Some(token) => match token?.ty() {
                    TokenType::Operator(Operator::Unary(UnaryOperator::RightParen)) => break,
                    TokenType::Operator(op) => op.try_into()?,
                    ty => {
                        return Err(Error::ParseError {
                            msg: format!("Unexpected {:?}", ty),
                        });
                    }
                },
                _ => break,
            };

            if let Some((l_bp, ())) = op.postfix_binding_power() {
                if l_bp < min_bp {
                    break;
                }
                self.lexer.next();

                lhs = TokenTree::Cons(op, vec![lhs]);
                continue;
            }

            if let Some((l_bp, r_bp)) = op.infix_binding_power() {
                if l_bp < min_bp {
                    break;
                }
                self.lexer.next();

                let rhs = self.parse_expr(r_bp)?;

                lhs = TokenTree::Cons(op, vec![lhs, rhs]);
                continue;
            }

            break;
        }

        Ok(lhs)
    }
}
