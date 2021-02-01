use std::iter::Map;
use std::str::Split;
use crate::tokens::Token::*;
use crate::tokens::Token;

pub fn lex(input: &str) -> Vec<Token> {
    input.split(" ").map(|tok|{
        match tok {
            "|" => Pipe,
            "&" => Fork,
            "&&" => LogAnd,
            "||" => LogOr,
            "<" => ReadFile,
            ">" => WriteFile,
            ">>" => AppendFile,
            "(" => LParen,
            ")" => RParen,
            t => Text(t.to_string())
        }
    }).collect()
}

#[test]
fn lexes_fork_command() {
    let text = "echo this is a --test & cat ./foo.bar > carp";

    let output = lex(text);

    assert_eq!(
        output,
        vec![
            Text("echo".to_string()),
            Text("this".to_string()),
            Text("is".to_string()),
            Text("a".to_string()),
            Text("--test".to_string()),
            Fork,
            Text("cat".to_string()),
            Text("./foo.bar".to_string()),
            WriteFile,
            Text("carp".to_string()),
        ]
    )
}