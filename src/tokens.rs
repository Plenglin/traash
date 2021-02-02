#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Text(String),
    Glob,
    Space,
    LogAnd,
    LogOr,
    Pipe,
    WriteFile,
    AppendFile,
    ReadFile,
    Semicolon,
    Fork,
    LParen,
    RParen,
}

impl Token {
    pub(crate) fn text(text: &str) -> Token {
        Token::Text(text.to_string())
    }
}
