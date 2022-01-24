pub mod parser;

pub mod commands;
pub use commands::Command;

pub mod cells;
pub use cells::Cells;

pub mod error;
pub use error::BraincluckError;

mod lexer;

/// Parses an input containing Brainfuck code.
pub fn bf_parse(input: &str) -> Result<Vec<Command>, BraincluckError> {
    parser::bf::BraincluckParser::new()
        .parse(lexer::Lexer::new(input))
        .map_err(|err| BraincluckError::ParseError(format!("{:?}", err)))
}
