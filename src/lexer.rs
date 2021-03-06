use crate::lexer::LexerError::{TrailingBackslash, UnknownOperator};
use crate::tokens::Token;
use crate::tokens::Token::*;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum LexerError {
    TrailingBackslash,
    UnknownOperator(String),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::TrailingBackslash => write!(f, "trailing backslash"),
            LexerError::UnknownOperator(op) => write!(f, "unknown operator {}", op),
        }
    }
}

fn is_operator(c: char) -> bool {
    return vec!['&', '|', '>', '<', ';'].contains(&c);
}

fn is_text(c: char) -> bool {
    match c {
        _ if c.is_alphanumeric() => true,
        '-' | '_' | '.' | '/' => true,
        _ => false,
    }
}

fn read_text(mut in_str: &str) -> Result<(&str, String), LexerError> {
    let mut acc: Vec<char> = vec![];
    loop {
        let c = match in_str.chars().next() {
            None => break,
            Some(c) => c,
        };
        if c == '\\' {
            in_str = &in_str[1..];
            match in_str.chars().next() {
                None => Err(TrailingBackslash)?,
                Some(c) => acc.push(c),
            };
        } else if !is_text(c) {
            break;
        } else {
            acc.push(c);
        }
        in_str = &in_str[1..];
    }
    Ok((&in_str, acc.into_iter().collect()))
}

fn read_operator(mut in_str: &str, operator: char) -> (&str, i8) {
    let mut repetitions = 0;
    while in_str.chars().next() == Some(operator) {
        repetitions += 1;
        in_str = &in_str[1..];
    }
    (in_str, repetitions)
}

fn skip_whitespace(mut in_str: &str) -> &str {
    while let Some(x) = in_str.chars().next() {
        if !x.is_whitespace() {
            break;
        }
        in_str = &in_str[1..];
    }
    in_str
}

pub fn lex(mut input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens: Vec<Token> = vec![];
    loop {
        let c = match input.chars().next() {
            None => break,
            Some(c) => c,
        };
        if c == '(' {
            tokens.push(LParen);
            input = &input[1..];
        } else if c == ')' {
            tokens.push(RParen);
            input = &input[1..];
        } else if c.is_whitespace() {
            input = skip_whitespace(input);
        } else if is_text(c) {
            let (t, text) = read_text(input)?;
            tokens.push(Token::Text(text));
            input = t;
        } else if is_operator(c) {
            let (t, repetitions) = read_operator(input, c);
            let token = match (c, repetitions) {
                ('&', 1) => Token::Fork,
                ('&', 2) => Token::LogAnd,
                ('|', 1) => Token::Pipe,
                ('|', 2) => Token::LogOr,
                ('>', 1) => Token::WriteFile,
                ('>', 2) => Token::AppendFile,
                (';', 1) => Token::Semicolon,
                _ => Err(UnknownOperator((0..repetitions).map(|_| c).collect()))?,
            };
            tokens.push(token);
            input = t;
        } else {
            panic!("Unknown character {}", c)
        }
    }
    Ok(tokens)
}

#[test]
fn lexes_empty() {
    let in_str = "";

    let output = lex(in_str).unwrap();

    assert_eq!(output, vec![]);
}

#[test]
fn lexes_whitespace() {
    let in_str = "\t";

    let output = lex(in_str).unwrap();

    assert_eq!(output, vec![]);
}

#[test]
fn lexes_fork_command() {
    let in_str = "echo this is a --test & cat ./foo.bar > carp";

    let output = lex(in_str).unwrap();

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

#[test]
fn escapes_chars() {
    let in_str = "echo this\\ is\\ one\\ token";

    let output = lex(in_str).unwrap();

    assert_eq!(
        output,
        vec![Token::text("echo"), Token::text("this is one token"),]
    )
}

#[test]
fn optional_whitespace() {
    let in_str = "echo;tok&&en&alpha||beta>>     \tend";

    let output = lex(in_str).unwrap();

    assert_eq!(
        output,
        vec![
            Token::text("echo"),
            Semicolon,
            Token::text("tok"),
            LogAnd,
            Token::text("en"),
            Fork,
            Token::text("alpha"),
            LogOr,
            Token::text("beta"),
            AppendFile,
            Token::text("end"),
        ]
    )
}

#[test]
fn lexes_parentheses() {
    let in_str = "(echo(   token(alpha)beta(";

    let output = lex(in_str).unwrap();

    assert_eq!(
        output,
        vec![
            LParen,
            Token::text("echo"),
            LParen,
            Token::text("token"),
            LParen,
            Token::text("alpha"),
            RParen,
            Token::text("beta"),
            LParen
        ]
    )
}

#[test]
fn errors_on_invalid_operators() {
    let in_str = "foo;;bar&alpha||beta";

    let output = lex(in_str);

    assert_eq!(output, Err(UnknownOperator(";;".to_string())))
}
