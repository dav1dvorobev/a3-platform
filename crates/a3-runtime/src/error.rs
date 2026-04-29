//! Contains `Error` and corresponding `Result`.

use http::header::{InvalidHeaderName, InvalidHeaderValue};
use rig::{client::ProviderClientError, tool::rmcp::McpClientError};

/// A result with a specified [Error] type.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents all possible errors.
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    IOError(#[from] std::io::Error),
    InvalidHeaderName(#[from] InvalidHeaderName),
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    McpClientError(#[from] McpClientError),
    ProviderClientError(#[from] ProviderClientError),
}
