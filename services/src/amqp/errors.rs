use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AmqpError {
    #[error("Internal Error")]
    InternalError,

    #[error("Failed to connect")]
    ConnectionError,

    #[error("Failed to create consumer")]
    ConsumerError,
}
