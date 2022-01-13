pub mod parser;

pub mod commands;
pub use commands::Command;

pub mod cells;
pub use cells::Cells;

pub mod error;
pub use error::BrainduckError;

mod lexer;

/// Parses an input containing Brainfuck code.
pub fn bf_parse(input: &str) -> Result<Vec<Command>, BrainduckError> {
    parser::bf::BrainduckParser::new()
        .parse(lexer::Lexer::new(input))
        .map_err(|err| BrainduckError::ParseError(format!("{:?}", err)))
}
