use crate::lexer::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn with_lexer(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }
}

impl Parser<'_> {
    pub fn parse(&mut self) {
        for token in self.lexer.next() {}
        todo!()
    }

    fn parse_expr(lexer: &mut Lexer) {
        todo!()
    }
}
