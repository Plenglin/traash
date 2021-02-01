#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SingleCommand {
    pub(crate) args: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Fork {
    pub first: Box<Command>,
    pub second: Box<Command>,
    pub wait: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Sequential {
    pub first: Box<Command>,
    pub second: Box<Command>,
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
    Nil,
    Single(SingleCommand),
    Fork(Fork),
    Sequential(Sequential),
    Pipe(Pipe),
    FileInput(FileInput),
    FileOutput(FileOutput),
}
