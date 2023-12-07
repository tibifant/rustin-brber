use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Missing field: {}", _0)]
    MissingField(String),
    #[error("Invalid type: {}", _0)]
    InvalidType(String),
}
#[derive(Error, Debug, PartialEq)]
pub enum RabbitMQConnectionError {
    #[error("Failed to open connection")]
    FailedToOpenConnection,
    #[error("Failed to register callback")]
    FailedToRegisterCallback,
    #[error("Failed to open channel")]
    FailedToOpenChannel,
    #[error("Failed to register callback for channel")]
    FailedToRegisterCallbackForChannel,
}