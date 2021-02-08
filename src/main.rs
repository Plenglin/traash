use crate::ast::single;
use crate::ast::Command::Single;
use crate::executor::StreamSet;

mod ast;
mod executor;
mod lexer;
mod parser;
mod tokens;

fn main() {
    let ss = StreamSet::std();
    executor::execute(single(vec!["echo".to_string(), "test".to_string()]), ss);
}
