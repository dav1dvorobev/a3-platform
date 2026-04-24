//! Contains `Error` and corresponding `Result`.

/// A result with a specified [Error] type.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents all possible errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {}
