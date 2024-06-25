use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AmqpError {
    #[error("Internal Error")]
    InternalError,

    #[error("Failed to connect")]
    ConnectionError,

    #[error("Failed to create consumer")]
    ConsumerError,

    #[error("Failed to unpack message")]
    UnpackMessageError,

    #[error("Unsupported message type")]
    UnsupportedMessageType,

    #[error("Ack message Error")]
    AckError,

    #[error("Process message Error")]
    ProcessMessage,
}
