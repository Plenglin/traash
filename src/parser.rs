use crate::lexer;
use crate::lexer::Token;
use crate::parser::Command::Single;
use std::ops::Index;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SingleCommand {
    args: Vec<String>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Fork {
    children: Vec<Box<Command>>,
    wait: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Sequential {
    children: Vec<Box<Command>>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pipe {
    src: Box<Command>,
    dst: Box<Command>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileInput {
    src: String,
    dst: Box<Command>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileOutput {
    src: Box<Command>,
    dst: String,
    append: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    Single(SingleCommand),
    Fork(Fork),
    Sequential(Sequential),
    Pipe(Pipe),
    FileInput(FileInput),
    FileOutput(FileOutput),
}

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

pub fn parse(mut tokens: &[lexer::Token]) -> Command {
    let (cmd, tokens) = match tokens.first() {
        Some(Token::Text(str)) => readNextSingleCommand(tokens),
        _ => (Single(SingleCommand { args: vec![] }), tokens)
    };
    cmd
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