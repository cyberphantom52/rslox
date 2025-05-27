use crate::{
    error::Error,
    token::{Token, TokenType},
};

pub struct Lexer<'a> {
    source_code: &'a str,
    byte_offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(stream: &'a str) -> Self {
        Self {
            source_code: stream,
            byte_offset: 0,
        }
    }

    pub fn line(&self) -> usize {
        self.source_code[..self.byte_offset].lines().count()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut iterator = self.source_code[self.byte_offset..].chars().peekable();

        let is_punct = |lexeme: char| -> bool {
            matches!(
                lexeme,
                '(' | ')' | '{' | '}' | ',' | '.' | ';' | '+' | '-' | '*'
            )
        };

        let is_alphabetic = |lexeme: char| -> bool { matches!(lexeme, 'a'..='z' | 'A'..='Z') };
        let is_alphanumeric =
            |lexeme: char| -> bool { is_alphabetic(lexeme) || lexeme.is_ascii_digit() };
        let is_literal =
            |lexeme: char| -> bool { is_alphanumeric(lexeme) || matches!(lexeme, '_') };

        while let Some(c) = iterator.next() {
            let cur_byte_offset = self.byte_offset;
            self.byte_offset += c.len_utf8();

            match c {
                c if c.is_whitespace() => continue,
                c if is_punct(c) => {}

                '!' | '=' | '<' | '>' => {
                    self.byte_offset += match iterator.peek() {
                        Some(&next) if next == '=' => next.len_utf8(),
                        _ => 0,
                    };
                }

                '/' => match iterator.peek() {
                    Some(&next) if next == '/' => {
                        let newline = iterator.position(|c| c == '\n');
                        match newline {
                            Some(pos) => self.byte_offset += pos + 1,
                            None => return None,
                        }
                        continue; // Skip to the next iteration
                    }
                    _ => {}
                },

                // Literals
                c if is_alphabetic(c) || c == '_' => {
                    let len = iterator.take_while(|&next| is_literal(next)).count();
                    self.byte_offset += len;
                }

                '"' => {
                    if let Some(end) = iterator.position(|c| c == '"') {
                        self.byte_offset += end + 1;
                    } else {
                        self.byte_offset = self.source_code.len();
                        return Some(Err(Error::LexingError {
                            ty: crate::error::LexingError::UnterminatedString,
                            line: self.line(),
                        }));
                    }
                }

                c if c.is_ascii_digit() => continue,
                _ => {
                    return Some(Err(Error::LexingError {
                        ty: crate::error::LexingError::UnexpectedCharacter(c),
                        line: self.line(),
                    }));
                }
            };

            let lexeme = &self.source_code[cur_byte_offset..self.byte_offset];
            let token_ty = TokenType::from(lexeme);
            return Some(Ok(Token::new(token_ty, lexeme.trim_matches('"'))));
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::token::Literal;

    use super::*;

    #[test]
    fn empty_input() {
        let input = "";
        let mut lexer = Lexer::new(input);
        assert!(lexer.next().is_none());
    }

    #[test]
    fn unexpected_characters() {
        let input = "@\n#$\n%^&\n*";
        let mut lexer = Lexer::new(input);

        match lexer.next() {
            Some(Err(e)) => {
                assert!(matches!(
                    e,
                    Error::LexingError {
                        ty: crate::error::LexingError::UnexpectedCharacter(_),
                        line: 1
                    }
                ));
            }
            o => panic!("Expected an error for unexpected character, got: {:?}", o),
        }
    }

    #[test]
    fn identifiers() {
        let input = "andy formless fo _ _123 _abc ab123
        abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            "andy",
            "formless",
            "fo",
            "_",
            "_123",
            "_abc",
            "ab123",
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_",
        ];

        for expected_lexeme in expected {
            match lexer.next() {
                Some(Ok(token)) => {
                    assert_eq!(token.lexeme(), expected_lexeme);
                    assert!(matches!(
                        token.ty(),
                        TokenType::Literal(Literal::Identifier)
                    ));
                }
                Some(Err(e)) => panic!("Unexpected error: {}", e),
                None => panic!("Expected more tokens, but got None"),
            }
        }
    }

    #[test]
    fn keywords() {
        let input = "and class else false for fun if nil or return super this true var while";
        let mut lexer = Lexer::new(input);

        let expected_keywords = vec![
            "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "return", "super",
            "this", "true", "var", "while",
        ];

        for expected_keyword in expected_keywords {
            match lexer.next() {
                Some(Ok(token)) => {
                    assert_eq!(token.lexeme(), expected_keyword);
                    assert!(matches!(token.ty(), TokenType::Keyword(_)));
                }
                Some(Err(e)) => panic!("Unexpected error: {}", e),
                None => panic!("Expected more tokens, but got None"),
            }
        }
    }

    #[test]
    fn strings() {
        // ""string""unter
        let input = "\"\"\"string\"\"unterminated string";
        let mut lexer = Lexer::new(input);
        let expected_strings = vec!["", "string"];

        for expected_string in expected_strings {
            match lexer.next() {
                Some(Ok(token)) => {
                    assert_eq!(token.lexeme(), expected_string);
                    assert!(matches!(token.ty(), TokenType::Literal(Literal::String)));
                }
                Some(Err(e)) => panic!("Unexpected error: {}", e),
                None => panic!("Expected more tokens, but got None"),
            }
        }

        assert!(matches!(
            lexer.next(),
            Some(Err(Error::LexingError {
                ty: crate::error::LexingError::UnterminatedString,
                line: 1
            }))
        ));
    }

    #[test]
    fn numbers() {
        let input = "123 123.456 .456 123.";
        // let mut lexer = Lexer::new(input);
    }

    #[test]
    fn punctuators() {
        let input = r#"(){};,+-*!===<=>=!=<>/."#;
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            "(", ")", "{", "}", ";", ",", "+", "-", "*", "!=", "==", "<=", ">=", "!=", "<", ">",
            "/", ".",
        ];

        for expected_token in expected_tokens {
            match lexer.next() {
                Some(Ok(token)) => {
                    assert_eq!(token.lexeme(), expected_token);
                    assert!(matches!(token.ty(), TokenType::Operator(_)));
                }
                Some(Err(e)) => panic!("Unexpected error: {}", e),
                None => panic!("Expected more tokens, but got None"),
            }
        }
    }

    #[test]
    fn whitespace() {
        let input = "space      tabs\t\t\t\tnewlines\n\n\n\n\nend";
        let mut lexer = Lexer::new(input);
        let expected_tokens = vec!["space", "tabs", "newlines", "end"];

        for expected_token in expected_tokens {
            match lexer.next() {
                Some(Ok(token)) => {
                    assert_eq!(token.lexeme(), expected_token);
                    assert!(matches!(
                        token.ty(),
                        TokenType::Literal(Literal::Identifier)
                    ));
                }
                Some(Err(e)) => panic!("Unexpected error: {}", e),
                None => panic!("Expected more tokens, but got None"),
            }
        }
    }
}
