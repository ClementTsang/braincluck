pub mod parser;

pub mod commands;
pub use commands::Command;

pub mod cells;
pub use cells::Cells;

mod lexer;

/// Parses an input containing Brainfuck code.
pub fn bf_parse(input: &str) -> Result<Vec<Command>, String> {
    parser::bf::BrainduckParser::new()
        .parse(lexer::Lexer::new(input))
        .map_err(|err| format!("{:?}", err))
}
