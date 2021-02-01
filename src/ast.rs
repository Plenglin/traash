#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SingleCommand {
    pub(crate) args: Vec<String>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Fork {
    children: Vec<Box<Command>>,
    wait: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Sequential {
    pub(crate) first: Box<Command>,
    pub(crate) second: Box<Command>
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
