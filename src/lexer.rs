use crate::lexer::LexerError::TrailingBackslash;
use crate::tokens::Token;
use crate::tokens::Token::*;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum LexerError {
    TrailingBackslash,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::TrailingBackslash => write!(f, "trailing backslash"),
        }
    }
}

fn is_operator(c: char) -> bool {}

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
        } else if c.is_whitespace() || !c.is_alphanumeric() {
            break;
        } else {
            acc.push(c);
        }
        in_str = &in_str[1..];
    }
    Ok((&in_str, acc.into_iter().collect()))
}

fn skip_whitespace(mut in_str: &str) -> &str {
    while let Some(x) = in_str.get(0..1) {
        match x {
            " " => 1,
            _ => break,
        };
        in_str = &in_str[1..];
    }
    in_str
}

pub fn lex(mut input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = vec![];
    loop {
        input = match &input.get(0..1) {
            Some(" ") => skip_whitespace(input),
            None => break,
            _ => {
                let (t, text) = read_text(input)?;
                tokens.push(Text(text));
                t
            }
        }
    }
    Ok(tokens)
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
    let in_str = "echo;token&alpha||beta";

    let output = lex(in_str).unwrap();

    assert_eq!(
        output,
        vec![
            Token::text("echo"),
            Semicolon,
            Token::text("token"),
            Fork,
            Token::text("alpha"),
            LogOr,
            Token::text("beta")
        ]
    )
}
