use crate::{error::Error, lexer::Lexer, token::TokenTree};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn with_lexer(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<TokenTree<'a>, Error> {
        todo!()
    }
}
