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
            Self::EmptyMessageId => f.write_str("Empty message ID."),
            Self::InvalidAddress => f.write_str("Invalid address."),
            Self::SendFailed => f.write_str("Failed to send message."),
            Self::ReceptionFailed => f.write_str("Failed to receive message."),
            Self::QueueFull => f.write_str("Queue is full."),
            Self::LockFailed(e) => write!(f, "Failed to acquire enqueue lock: {}", e),
            Self::ServiceNotFound => f.write_str("Service not found."),
            Self::ProcessorNotFound => f.write_str("Processor not found."),
        }
    }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{
            sync::{
                Arc,
                Mutex,
            },
            thread::spawn,
        },
    };

    #[test]
    fn error_display() {
        let empty_msg_id = MyErr::EmptyMessageId;
        let invalid_addr = MyErr::InvalidAddress;
        let queue_full = MyErr::QueueFull;

        assert_eq!(empty_msg_id.to_string(), "Empty message ID.");
        assert_eq!(invalid_addr.to_string(), "Invalid address.");
        assert_eq!(queue_full.to_string(), "Queue is full.");
    }

    #[test]
    fn poison_error() {
        let data = Arc::new(Mutex::new(0));

        // Trigger Poison
        let data_dup = data.clone();
        let _ = spawn(move || {
            let _lock = data_dup.lock().unwrap();
            panic!("poisoned")
        })
        .join();

        let err: MyErr = data
            .lock()
            .unwrap_err()
            .into();

        assert!(matches!(err, MyErr::LockFailed(_)))
    }
}
