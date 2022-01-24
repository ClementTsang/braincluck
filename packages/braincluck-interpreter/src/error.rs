use std::num::TryFromIntError;

use thiserror::Error;

/// The errors that can result while trying to interpret Brainfuck code.
#[derive(Error, Debug)]
pub enum BraincluckError {
    #[error("parsing error: `{0}`")]
    ParseError(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ConversionError(#[from] TryFromIntError),
    #[error(transparent)]
    StringFormatError(#[from] std::fmt::Error),
}
