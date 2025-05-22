struct Lexer {
    lines: Vec<String>,
}

impl Lexer {
    pub fn new(stream: String) -> Self {
        Self {
            lines: stream.lines().map(|line| line.to_string()).collect(),
        }
    }
}
