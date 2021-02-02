use crate::tokens::Token;
use crate::tokens::Token::*;

pub fn lex(input: &str) -> Vec<Token> {
    input
        .split(" ")
        .map(|tok| match tok {
            "|" => Pipe,
            "&" => Fork,
            "&&" => LogAnd,
            "||" => LogOr,
            "<" => ReadFile,
            ">" => WriteFile,
            ">>" => AppendFile,
            "(" => LParen,
            ")" => RParen,
            t => Text(t.to_string()),
        })
        .collect()
}

#[test]
fn lexes_fork_command() {
    let in_str = "echo this is a --test & cat ./foo.bar > carp";

    let output = lex(in_str);

    assert_eq!(
        output,
        vec![
            Token::text("echo"),
            Token::text("this"),
            Token::text("is"),
            Token::text("a"),
            Token::text("--test"),
            Fork,
            Token::text("cat"),
            Token::text("./foo.bar"),
            WriteFile,
            Token::text("carp"),
        ]
    )
}
