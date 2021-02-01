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
