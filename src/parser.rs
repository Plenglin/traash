use std::ops::Index;

use crate::ast::{Command, Sequential, SingleCommand};
use crate::parser::Command::Single;
use crate::tokens::Token;
use crate::ast::Command::Nil;

fn read_next_single_command(mut tokens: &[Token]) -> (Command, &[Token]) {
    let mut args = vec![];

    loop {
        match tokens.first() {
            Some(Token::Text(str)) => args.push(str.to_string()),
            _ => break
        }
        tokens = &tokens[1..];
    }

    (if args.is_empty() { Nil } else { Command::Single(SingleCommand { args }) }, tokens)
}

pub fn parse(mut tokens: &[Token]) -> Command {
    if tokens.is_empty() {
        return Nil
    }

    let mut stack: Vec<Command> = vec![];

    while !tokens.is_empty() {
        match tokens.first() {
            Some(Token::Text(str)) => {
                let (cmd, t) = read_next_single_command(tokens);
                tokens = t;
                stack.push(cmd);
            }
            Some(Token::Semicolon) => {
                let first = stack.pop().unwrap();
                let (cmd, t) = read_next_single_command(&tokens[1..]);
                tokens = t;
                stack.push(Command::Sequential(Sequential { first: Box::new(first), second: Box::new(cmd) }));
            }
            _ => {}
        };
    }
    stack.pop().unwrap()
}

#[test]
fn parses_empty_tokens() {
    let tokens = vec![];
    let result = parse(tokens.as_slice());

    assert_eq!(result, Command::Nil);
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
