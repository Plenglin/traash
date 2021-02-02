use std::ops::Index;

use crate::ast::Command::Nil;
use crate::ast::{Command, Sequential, SingleCommand};
use crate::parser::Command::Single;
use crate::parser::Symbol::Text;
use crate::tokens::Token;

enum Symbol {
    Text(String),
    Command(Command),
}

struct Parser<'a> {
    tokens: &'a [Token],
    stack: Vec<Symbol>,
}

impl Parser<'_> {
    fn new(tokens: &[Token]) -> Parser {
        Parser {
            tokens,
            stack: vec![],
        }
    }
}

impl Parser<'_> {
    fn reduce_single(self: &mut Self) -> SingleCommand {
        let mut args: Vec<String> = vec![];
        while let Some(Symbol::Text(str)) = self.stack.last() {
            args.push(str.clone());
            self.stack.pop();
        }
        args.reverse();
        SingleCommand { args }
    }

    fn parse(self: &mut Self) -> Command {
        loop {
            let token = match self.tokens.first() {
                Some(t) => t,
                None => break,
            };

            match token {
                Token::Text(str) => self.stack.push(Text(str.clone())),
                Token::Fork => {
                    let cmd = self.reduce_single();
                }
                _ => {}
            }

            self.tokens = &self.tokens[1..];
        }
        let last = Symbol::Command(Command::Single(self.reduce_single()));
        self.stack.push(last);
        match self.stack.pop() {
            Some(Symbol::Command(cmd)) => cmd,
            None => Nil,
            _ => panic!(),
        }
    }
}

pub fn parse<'a>(tokens: &'a [Token]) -> Command {
    let mut parser = Parser::new(tokens);
    parser.parse()
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

    assert_eq!(
        result,
        Command::Single(SingleCommand {
            args: vec!["echo".to_string(), "foo".to_string()]
        })
    );
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
            first: Box::new(Command::Single(SingleCommand {
                args: vec!["echo".to_string(), "foo".to_string()]
            })),
            second: Box::new(Command::Sequential(Sequential {
                first: Box::new(Command::Single(SingleCommand {
                    args: vec!["echo".to_string(), "bar".to_string()]
                })),
                second: Box::new(Command::Single(SingleCommand {
                    args: vec!["echo".to_string(), "spam".to_string()]
                })),
            })),
        }),
    );
}
