//! Contains `Error` and corresponding `Result`.

/// A result with a specified [Error] type.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents all possible errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("missing required manifest field: {0}")]
    MissingField(&'static str),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
