use crate::ast::Command::Nil;
use crate::ast::{binary, fork, sequential, single, BinaryOp, Command, SingleCommand};
use crate::parser::Command::Single;
use crate::tokens::Token;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Symbol {
    Text(String),
    BinaryOp(Command, BinaryOp),
    Command(Command),
    LParen,
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
    fn reduce_single(self: &mut Self) -> Command {
        let mut args: Vec<String> = vec![];
        while let Some(Symbol::Text(str)) = self.stack.last() {
            args.push(str.clone());
            self.stack.pop();
        }
        if args.is_empty() {
            Nil
        } else {
            args.reverse();
            Single(SingleCommand { args })
        }
    }

    fn reduce(self: &mut Self) -> Command {
        loop {
            let push = match self.stack.pop() {
                Some(Symbol::Text(text)) => {
                    self.stack.push(Symbol::Text(text));
                    let command = self.reduce_single();
                    Symbol::Command(command)
                }
                Some(Symbol::Command(cmd)) => match self.stack.pop() {
                    Some(Symbol::BinaryOp(left, op)) => Symbol::Command(binary(op, left, cmd)),
                    Some(Symbol::LParen) | None => {
                        return cmd;
                    }
                    _ => continue,
                },
                Some(Symbol::BinaryOp(left, op)) => Symbol::Command(binary(op, left, Nil)),
                None => {
                    return Nil;
                }
                Some(x) => panic!("Encountered a {:#?}", x),
            };
            self.stack.push(push);
        }
    }

    fn parse(self: &mut Self) -> Command {
        // Read through tokens
        loop {
            let token = match self.tokens.first() {
                Some(t) => t,
                None => break,
            };

            if let Token::Text(str) = token {
                self.stack.push(Symbol::Text(str.clone()))
            } else if let Some(op) = BinaryOp::from(token) {
                let command = self.reduce();
                self.stack.push(Symbol::BinaryOp(command, op));
            }

            self.tokens = &self.tokens[1..];
        }

        // Clear out the stack
        self.reduce()
    }
}

pub fn parse(tokens: &[Token]) -> Command {
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
    let tokens = vec![Token::text("echo"), Token::text("foo")];
    let result = parse(tokens.as_slice());

    assert_eq!(
        result,
        Command::Single(SingleCommand {
            args: vec!["echo".to_string(), "foo".to_string()]
        })
    );
}

#[test]
fn parses_chained_binary_command() {
    let tokens = vec![
        Token::text("echo"),
        Token::text("foo"),
        Token::Semicolon,
        Token::text("echo"),
        Token::text("bar"),
        Token::Semicolon,
        Token::text("echo"),
        Token::text("spam"),
    ];
    let result = parse(tokens.as_slice());

    assert_eq!(
        result,
        sequential(
            sequential(
                single(vec!["echo".to_string(), "foo".to_string()]),
                single(vec!["echo".to_string(), "bar".to_string()]),
            ),
            single(vec!["echo".to_string(), "spam".to_string()])
        )
    );
}

#[test]
fn parses_binary_command_with_trailing_op() {
    let tokens = vec![
        Token::text("echo"),
        Token::text("foo"),
        Token::Semicolon,
        Token::text("echo"),
        Token::Fork,
    ];
    let result = parse(tokens.as_slice());

    assert_eq!(
        result,
        fork(
            sequential(
                single(vec!["echo".to_string(), "foo".to_string()]),
                single(vec!["echo".to_string()]),
            ),
            Nil
        )
    );
}
