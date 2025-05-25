use crate::token::{Literal, Operator, Token, TokenType, UnaryOperator};

pub struct Lexer {
    lines: Vec<String>,
}

impl Lexer {
    pub fn new(stream: String) -> Self {
        Self {
            lines: stream.lines().map(|line| line.to_string()).collect(),
        }
    }

    pub fn scan(&self, line: &str, line_number: usize) -> Vec<Token> {
        let mut iterator = line.chars().peekable();
        let mut tokens = Vec::new();
        while let Some(c) = iterator.next() {
            let mut lexeme = c.to_string();
            let token_type = match c {
                // Operators
                '(' | ')' | '{' | '}' | ',' | '.' | ';' | '+' | '-' | '*' => {
                    TokenType::from(lexeme.as_str())
                }
                '!' | '=' | '<' | '>' => {
                    if let Some(&next) = iterator.peek() {
                        if next == '=' {
                            lexeme.push(iterator.next().unwrap())
                        }
                    }
                    TokenType::from(lexeme.as_str())
                }
                '/' => match iterator.peek() {
                    Some(&next) if next == '/' => return tokens,
                    _ => TokenType::Operator(Operator::Unary(UnaryOperator::Slash)),
                },

                // Literals
                '"' => {
                    lexeme = String::new();
                    while let Some(next) = iterator.next() {
                        if next == '"' {
                            break;
                        }
                        lexeme.push(next);
                    }
                    TokenType::Literal(Literal::String)
                }

                c if c.is_digit(10) => {
                    while let Some(next) = iterator.peek() {
                        if !next.is_digit(10) {
                            break;
                        }
                        lexeme.push(iterator.next().unwrap());
                    }
                    TokenType::Literal(Literal::Number)
                }

                c if c.is_alphabetic() || c == '_' => {
                    while let Some(next) = iterator.peek() {
                        if !next.is_alphanumeric() {
                            break;
                        }
                        lexeme.push(iterator.next().unwrap());
                    }

                    let ty = TokenType::from(lexeme.as_str());
                    if ty == TokenType::Invalid {
                        TokenType::Literal(Literal::Identifier)
                    } else {
                        ty
                    }
                }

                c if c.is_whitespace() => continue,

                _ => TokenType::Invalid,
            };

            tokens.push(Token::new(token_type, lexeme, line_number));
        }
        tokens
    }

    pub fn consume(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        for (line_index, line) in self.lines.iter().enumerate() {
            tokens.extend(self.scan(line, line_index + 1));
        }
        tokens
    }
}
