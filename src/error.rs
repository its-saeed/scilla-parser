use std::string::FromUtf8Error;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Failed to map {0} to any rust type")]
    FailedToMapScillaTypeToRust(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    LexprParseError(#[from] lexpr::parse::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
}
