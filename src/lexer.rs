use crate::{
    error::Error,
    token::{Keyword, Literal, Operator, Token, TokenType, UnaryOperator},
};

pub struct Lexer<'a> {
    remaining: &'a str,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.remaining.chars().peekable();

        let is_punct = |lexeme: char| -> bool {
            matches!(
                lexeme,
                '(' | ')' | '{' | '}' | ',' | '.' | ';' | '+' | '-' | '*'
            )
        };

        let is_alphabetic = |lexeme: char| -> bool { matches!(lexeme, 'a'..='z' | 'A'..='Z') };
        let is_digit = |lexeme: char| -> bool { matches!(lexeme, '0'..='9') };
        let is_alphanumeric = |lexeme: char| -> bool { is_alphabetic(lexeme) || is_digit(lexeme) };
        let is_literal =
            |lexeme: char| -> bool { is_alphanumeric(lexeme) || matches!(lexeme, '_') };

        let mut lexeme: &str = "";
        while let Some(c) = iterator.next() {
            let token_ty = match c {
                c if c.is_whitespace() => {
                    // Skip whitespace characters
                    self.remaining = &self.remaining[1..];
                    continue;
                }

                // Operators
                c if is_punct(c) => {
                    lexeme = &self.remaining[0..1];
                    TokenType::from(lexeme)
                }

                '!' | '=' | '<' | '>' => {
                    let split_index = match iterator.peek() {
                        Some(&next) if next == '=' => 2,
                        _ => 1,
                    };
                    lexeme = &self.remaining[0..split_index];

                    TokenType::from(lexeme)
                }

                '/' => match iterator.peek() {
                    Some(&next) if next == '/' => {
                        let newline = iterator.position(|c| c == '\n');

                        match newline {
                            Some(pos) => self.remaining = &self.remaining[pos..],
                            None => return None,
                        }
                        continue; // Skip to the next iteration
                    }
                    _ => {
                        lexeme = &self.remaining[0..1];
                        TokenType::Operator(Operator::Unary(UnaryOperator::Slash))
                    }
                },

                // Literals
                c if is_alphabetic(c) || c == '_' => {
                    let len = iterator
                        .clone()
                        .take_while(|&next| is_literal(next))
                        .count();
                    lexeme = &self.remaining[0..=len];

                    if let Ok(kw) = Keyword::try_from(lexeme) {
                        TokenType::Keyword(kw)
                    } else {
                        TokenType::Literal(Literal::Identifier)
                    }
                }

                '"' => {
                    let len;
                    if let Some(end) = iterator.position(|c| c == '"') {
                        len = end + 1;
                    } else {
                        return Some(Err(Error::LexingError {
                            ty: crate::error::LexingError::UnterminatedString(String::new()),
                            line: 1,
                        }));
                    }

                    lexeme = &self.remaining[0..=len];
                    TokenType::Literal(Literal::String)
                }

                c if is_digit(c) => TokenType::Literal(Literal::Number(0f64)),
                _ => {
                    return Some(Err(Error::LexingError {
                        ty: crate::error::LexingError::UnexpectedCharacter(c),
                        line: 1,
                    }));
                }
            };

            self.remaining = &self.remaining[lexeme.len()..];
            return Some(Ok(Token::new(token_ty, lexeme.trim_matches('"'))));
        }
        None
    }
}

impl<'a> Lexer<'a> {
    pub fn new(stream: &'a str) -> Self {
        Self { remaining: stream }
    }
}
