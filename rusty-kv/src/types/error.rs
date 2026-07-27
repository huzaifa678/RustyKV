#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyCommand,
    UnknownCommand,
    InvalidArgumentCommand
}