#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    Jump(Vec<Command>),
}
