use crate::ast::Command::Nil;
use crate::ast::{binary, fork, log_and, sequential, single, BinaryOp, Command, SingleCommand};
use crate::parser::Command::Single;
use crate::parser::ParserError::ExtraRParen;
use crate::tokens::Token;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Symbol {
    Text(String),
    BinaryOp(Command, BinaryOp),
    Command(Command),
    LParen,
}

#[derive(Debug)]
pub enum ParserError {
    ExtraRParen,
    MissingRParen,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::ExtraRParen => write!(f, "there was an extra right parenthesis"),
            ParserError::MissingRParen => write!(f, "there was an missing right parenthesis"),
        }
    }
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
                    Some(Symbol::LParen) => {
                        self.stack.push(Symbol::LParen);
                        return cmd;
                    }
                    None => {
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

    fn parse(self: &mut Self) -> Result<Command, ParserError> {
        // Read through tokens
        loop {
            let token = match self.tokens.first() {
                Some(t) => t,
                None => break,
            };

            let push = if let Some(op) = BinaryOp::from(token) {
                let command = self.reduce();
                Symbol::BinaryOp(command, op)
            } else {
                match token {
                    Token::Text(str) => Symbol::Text(str.clone()),
                    Token::LParen => Symbol::LParen,
                    Token::RParen => {
                        let command = self.reduce();
                        match self.stack.pop() {
                            Some(Symbol::LParen) => Symbol::Command(command),
                            _ => Err(ExtraRParen)?,
                        }
                    }
                    _ => panic!(),
                }
            };
            self.stack.push(push);

            self.tokens = &self.tokens[1..];
        }

        // Clear out the stack
        Ok(self.reduce())
    }
}

pub fn parse(tokens: &[Token]) -> Result<Command, ParserError> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn parses_empty_tokens() {
    let tokens = vec![];
    let result = parse(tokens.as_slice()).unwrap();

    assert_eq!(result, Command::Nil);
}

#[test]
fn parses_single_command() {
    let tokens = vec![Token::text("echo"), Token::text("foo")];
    let result = parse(tokens.as_slice()).unwrap();

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
    let result = parse(tokens.as_slice()).unwrap();

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
fn reorders_binary_chain_with_paren() {
    let tokens = vec![
        Token::text("echo"),
        Token::text("foo"),
        Token::Semicolon,
        Token::LParen,
        Token::text("echo"),
        Token::text("bar"),
        Token::Semicolon,
        Token::text("echo"),
        Token::text("spam"),
        Token::RParen,
    ];
    let result = parse(tokens.as_slice()).unwrap();

    assert_eq!(
        result,
        sequential(
            single(vec!["echo".to_string(), "foo".to_string()]),
            sequential(
                single(vec!["echo".to_string(), "bar".to_string()]),
                single(vec!["echo".to_string(), "spam".to_string()])
            )
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
    let result = parse(tokens.as_slice()).unwrap();

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

#[test]
fn parses_happy_command_with_parentheses() {
    let tokens = vec![
        Token::LParen,
        Token::text("uptime"),
        Token::Semicolon,
        Token::text("echo"),
        Token::RParen,
    ];
    let result = parse(tokens.as_slice()).unwrap();

    assert_eq!(
        result,
        sequential(
            single(vec!["uptime".to_string()]),
            single(vec!["echo".to_string()]),
        )
    );
}

#[test]
fn parses_happy_command_with_nested_parentheses() {
    let tokens = vec![
        Token::text("uptime"),
        Token::Semicolon,
        Token::LParen,
        Token::LParen,
        Token::text("echo"),
        Token::Fork,
        Token::RParen,
        Token::LogAnd,
        Token::text("apt"),
        Token::RParen,
    ];
    let result = parse(tokens.as_slice()).unwrap();

    assert_eq!(
        result,
        sequential(
            single(vec!["uptime".to_string()]),
            log_and(
                fork(single(vec!["echo".to_string()]), Nil),
                single(vec!["echo".to_string()]),
            )
        )
    );
}
