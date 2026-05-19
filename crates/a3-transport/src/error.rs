//! Contains `Error` and corresponding `Result`.

/// A result with a specified [Error] type.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents all possible errors.
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    #[error("address error: {0}")]
    AddressError(&'static str),
    EnvironmentError(#[from] std::env::VarError),
    NatsConnectError(#[from] async_nats::ConnectError),
    NatsSubscribeError(#[from] async_nats::SubscribeError),
    NatsPublishError(#[from] async_nats::PublishError),
    JsonError(#[from] serde_json::Error),
}
