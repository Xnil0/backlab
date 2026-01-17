use std::{
    fmt,
    sync::PoisonError,
};

pub type MyResult<T> = Result<T, MyErr>;

#[derive(Debug)]
pub enum MyErr {
    InvalidMessage,
    InvalidAddress,
    WrongStrategy,
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
            Self::InvalidMessage => f.write_str("Invalid message."),
            Self::InvalidAddress => f.write_str("Invalid address."),
            Self::WrongStrategy => f.write_str("Wrong balancing strategy."),
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
        let empty_msg_id = MyErr::InvalidMessage;
        let invalid_addr = MyErr::InvalidAddress;
        let wrong_strategy = MyErr::WrongStrategy;
        let queue_full = MyErr::QueueFull;
        let lock_failed = MyErr::LockFailed("test".to_string());
        let service_not_found = MyErr::ServiceNotFound;
        let processor_not_found = MyErr::ProcessorNotFound;

        assert_eq!(empty_msg_id.to_string(), "Invalid message.");
        assert_eq!(invalid_addr.to_string(), "Invalid address.");
        assert_eq!(
            wrong_strategy.to_string(),
            "Wrong balancing strategy."
        );
        assert_eq!(queue_full.to_string(), "Queue is full.");
        assert_eq!(
            lock_failed.to_string(),
            "Failed to acquire enqueue lock: test"
        );
        assert_eq!(
            service_not_found.to_string(),
            "Service not found."
        );
        assert_eq!(
            processor_not_found.to_string(),
            "Processor not found."
        );
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
