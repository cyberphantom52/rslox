use crate::token::{Operator, Token, TokenType, UnaryOperator};

struct Lexer {
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
            let token = match c {
                '(' | ')' | '{' | '}' | ',' | '.' | ';' | '+' | '-' | '*' => Token::new(
                    TokenType::from(c.to_string().as_str()),
                    c.to_string(),
                    line_number,
                ),
                '!' | '=' | '<' | '>' => {
                    let mut lexeme = c.to_string();
                    if let Some(&next) = iterator.peek() {
                        if next == '=' {
                            lexeme.push(iterator.next().unwrap())
                        }
                    }

                    Token::new(TokenType::from(lexeme.as_str()), lexeme, line_number)
                }
                '/' => match iterator.peek() {
                    Some(&next) if next == '/' => return tokens,
                    _ => Token::new(
                        TokenType::Operator(Operator::Unary(UnaryOperator::Slash)),
                        c.to_string(),
                        line_number,
                    ),
                },
                ' ' | '\r' | '\t' => todo!(),
                _ => panic!(""),
            };

            tokens.push(token);
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
