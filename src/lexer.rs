use std::iter::Map;
use std::str::Split;

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
    Fork
}

pub fn lex(input: &str) -> Vec<Token> {
    input.split(" ").map(|tok|{
        match tok {
            "|" => Token::Pipe,
            "&" => Token::Fork,
            "&&" => Token::LogAnd,
            "||" => Token::LogOr,
            "<" => Token::ReadFile,
            ">" => Token::WriteFile,
            ">>" => Token::AppendFile,
            t => Token::Text(t.to_string())
        }
    }).collect()
}
