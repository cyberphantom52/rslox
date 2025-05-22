pub enum TokenType {
    Keyword,
    Literal,
    Operator,
}

pub struct Token {
    ty: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, line: usize) -> Self {
        Self { ty, lexeme, line }
    }
}
