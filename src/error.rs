use std::string::FromUtf8Error;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("The requested entry {0} does not exist in the given S-expression")]
    NoSuchEntryInSexp(String),

    #[error("Comptype is not transition. It's {0}")]
    CompTypeIsNotTransition(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    LexprParseError(#[from] lexpr::parse::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
}
