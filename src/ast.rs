use crate::tokens::Token;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SingleCommand {
    pub(crate) args: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BinaryOp {
    Seq,
    Fork,
    Pipe,
    LogAnd,
    LogOr,
}

impl BinaryOp {
    pub fn from(token: &Token) -> Option<BinaryOp> {
        match token {
            Token::LogAnd => Some(BinaryOp::LogAnd),
            Token::LogOr => Some(BinaryOp::LogOr),
            Token::Pipe => Some(BinaryOp::Pipe),
            Token::Semicolon => Some(BinaryOp::Seq),
            Token::Fork => Some(BinaryOp::Fork),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub first: Box<Command>,
    pub second: Box<Command>,
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
    Nil,
    Single(SingleCommand),
    BinaryExpr(BinaryExpr),
    FileInput(FileInput),
    FileOutput(FileOutput),
}

pub fn single(args: Vec<String>) -> Command {
    Command::Single(SingleCommand { args })
}

pub fn binary(op: BinaryOp, a: Command, b: Command) -> Command {
    Command::BinaryExpr(BinaryExpr {
        op,
        first: Box::new(a),
        second: Box::new(b),
    })
}

pub fn sequential(a: Command, b: Command) -> Command {
    binary(BinaryOp::Seq, a, b)
}

pub fn fork(a: Command, b: Command) -> Command {
    binary(BinaryOp::Fork, a, b)
}

pub fn log_and(a: Command, b: Command) -> Command {
    binary(BinaryOp::LogAnd, a, b)
}

pub fn log_or(a: Command, b: Command) -> Command {
    binary(BinaryOp::LogOr, a, b)
}

pub fn pipe(a: Command, b: Command) -> Command {
    binary(BinaryOp::Pipe, a, b)
}
