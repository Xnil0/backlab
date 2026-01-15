use std::{
    fmt,
    sync::PoisonError,
};

pub type MyResult<T> = Result<T, MyErr>;

#[derive(Debug)]
pub enum MyErr {
    EmptyMessageId,
    InvalidAddress,
    SendFailed,
    ReceptionFailed,
    InvalidQueueDurability,
    InvalidQueueDeliveryGuarantee,
    InvalidQueueOrdering,
    InvalidQueueProvider,
    QueueFull,
    LockFailed(String),
    ServiceNotFound,
    ProcessorNotFound,
}

impl<T> From<PoisonError<T>> for MyErr {
    fn from(e: PoisonError<T>) -> Self { Self::LockFailed(format!("Lock Poisoned: {e}")) }
}

impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMessageId => f.write_str("Empty Message ID."),
            Self::InvalidAddress => f.write_str("Invalid address."),
            Self::SendFailed => f.write_str("Failed to send message."),
            Self::ReceptionFailed => f.write_str("Failed to receive message."),
            Self::InvalidQueueDurability => f.write_str("Invalid queue durability option."),
            Self::InvalidQueueDeliveryGuarantee => f.write_str("Invalid delivery guarantee."),
            Self::InvalidQueueOrdering => f.write_str("Invalid queue ordering option."),
            Self::InvalidQueueProvider => f.write_str("Invalid queue provider."),
            Self::QueueFull => f.write_str("Queue is full."),
            Self::LockFailed(e) => write!(f, "Failed to acquire enqueue lock: {}", e),
            Self::ServiceNotFound => f.write_str("Service not found."),
            Self::ProcessorNotFound => f.write_str("Processor not found."),
        }
    }
}
