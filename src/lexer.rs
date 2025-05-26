use crate::{
    error::Error,
    token::{Literal, Operator, Token, TokenType, UnaryOperator},
};

pub struct Lexer<'a> {
    remaining: &'a str,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.remaining.chars().peekable();

        let is_punctuation = |lexeme: char| -> bool {
            matches!(
                lexeme,
                '(' | ')' | '{' | '}' | ',' | '.' | ';' | '+' | '-' | '*' | '!' | '=' | '<' | '>'
            )
        };
        let is_alphabetic = |lexeme: char| -> bool { matches!(lexeme, 'a'..='z' | 'A'..='Z') };
        let is_digit = |lexeme: char| -> bool { matches!(lexeme, '0'..='9') };
        let is_alphanumeric = |lexeme: char| -> bool { is_alphabetic(lexeme) || is_digit(lexeme) };
        let is_literal =
            |lexeme: char| -> bool { is_alphanumeric(lexeme) || matches!(lexeme, '_' | '.') };

        while let Some(c) = iterator.next() {
            let mut lexeme = c.to_string();
            let token_type = match c {
                // Ignore Whitespace
                c if c.is_whitespace() => continue,

                // Operators
                c if is_punctuation(c) => {
                    if let Some(&next) = iterator.peek() {
                        if next == '=' {
                            lexeme.push(iterator.next().unwrap())
                        }
                    }

                    TokenType::from(lexeme.as_str())
                }

                '/' => match iterator.peek() {
                    Some(&next) if next == '/' => return None,
                    _ => TokenType::Operator(Operator::Unary(UnaryOperator::Slash)),
                },

                // Literals
                c if is_literal(c) => {
                    while let Some(&next) = iterator.peek() {
                        if !is_literal(next) {
                            break;
                        }
                        lexeme.push(iterator.next().unwrap());
                    }

                    // let ty = TokenType::from(lexeme.as_str());
                    // if ty == TokenType::Invalid {
                    // } else {
                    // ty
                    // }
                    TokenType::Literal(Literal::Identifier)
                }

                // '"' => {
                //     lexeme = String::new();
                //     while let Some(next) = iterator.next() {
                //         if next == '"' {
                //             break;
                //         }
                //         lexeme.push(next);
                //     }
                //     TokenType::Literal(Literal::String)
                // }

                // c if is_digit(c) => {
                //     while let Some(&next) = iterator.peek() {
                //         if !is_digit(next) {
                //             break;
                //         }
                //         lexeme.push(iterator.next().unwrap());
                //     }
                //     TokenType::Literal(Literal::Number)
                // }
                _ => {
                    return Some(Err(Error::LexingError { lexeme, line: 1 }));
                }
            };

            self.remaining = &self.remaining[1..];
            return Some(Ok(Token::new(token_type, lexeme)));
        }
        None
    }
}

impl<'a> Lexer<'a> {
    pub fn new(stream: &'a str) -> Self {
        Self { remaining: stream }
    }
}
