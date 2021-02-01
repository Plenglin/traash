use std::ops::Index;

use crate::ast::{Command, Sequential, SingleCommand};
use crate::parser::Command::Single;
use crate::tokens::Token;

fn readNextSingleCommand(tokens: &[Token]) -> (Command, &[Token]) {
    let mut i = 0;
    let mut args = vec![];

    loop {
        if i >= tokens.len() {
            break;
        }

        match &tokens[i] {
            Token::Text(str) => args.push(str.to_string()),
            _ => break
        }
        i += 1;
    }

    (Command::Single(SingleCommand { args }), &tokens[i..])
}

pub fn parse(mut tokens: &[Token]) -> Command {
    let mut stack: Vec<Command> = vec![];

    while !tokens.is_empty() {
        match tokens.first() {
            Some(Token::Text(str)) => {
                let (cmd, t) = readNextSingleCommand(tokens);
                tokens = t;
                stack.push(cmd);
            }
            Some(Token::Semicolon) => {
                let first = stack.pop().unwrap();
            }
            _ => {}
        };
    }
    stack.pop().unwrap()
}

#[test]
fn parses_single_command() {
    let tokens = vec![
        Token::Text("echo".to_string()),
        Token::Text("foo".to_string()),
    ];
    let result = parse(tokens.as_slice());

    assert_eq!(result, Command::Single(SingleCommand {
        args: vec!["echo".to_string(), "foo".to_string()]
    }));
}

#[test]
fn parses_sequential_command() {
    let tokens = vec![
        Token::Text("echo".to_string()),
        Token::Text("foo".to_string()),
        Token::Semicolon,
        Token::Text("echo".to_string()),
        Token::Text("bar".to_string()),
        Token::Semicolon,
        Token::Text("echo".to_string()),
        Token::Text("spam".to_string()),
    ];
    let result = parse(tokens.as_slice());

    assert_eq!(
        result,
        Command::Sequential(Sequential {
            first: Box::new(Command::Sequential(Sequential {
                first: Box::new(Command::Single(SingleCommand {
                    args: vec!["echo".to_string(), "foo".to_string()]
                })),
                second: Box::new(Command::Single(SingleCommand {
                    args: vec!["echo".to_string(), "bar".to_string()]
                })),
            })),
            second: Box::new(Command::Single(SingleCommand {
                args: vec!["echo".to_string(), "spam".to_string()]
            })),
        }),
    );
}
