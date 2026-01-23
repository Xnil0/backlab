use {
    crate::Envelope,
    std::{
        fmt,
        result,
        sync::PoisonError,
    },
};

/// Result type for bakbon operations.
///
/// Thin wrapper around `std::result::Result` with [`Error`](crate::Error)
/// as Error type.
pub type Result<T> = result::Result<T, Error>;

/// Errors that can occur in bakbon operations.
///
/// Covers invalid message, invalid address, wrong balancing strategy,
/// queue full, lock failed, service not found, and processor not found.
#[derive(Debug)]
pub enum Error {
    InvalidMessage,
    InvalidAddress,
    WrongStrategy,
    QueueFull(Envelope),
    LockFailed(String),
    ServiceNotFound,
    ProcessorNotFound,
}

impl<T> From<PoisonError<T>> for Error {
    fn from(e: PoisonError<T>) -> Self { Self::LockFailed(format!("Lock Poisoned: {e}")) }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMessage => f.write_str("Invalid message."),
            Self::InvalidAddress => f.write_str("Invalid address."),
            Self::WrongStrategy => f.write_str("Wrong balancing strategy."),
            Self::QueueFull(msg) => write!(f, "Queue is full: {:?}", msg),
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
        crate::Address,
        bytes::Bytes,
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
        let empty_msg_id = Error::InvalidMessage;
        let invalid_addr = Error::InvalidAddress;
        let wrong_strategy = Error::WrongStrategy;
        let lock_failed = Error::LockFailed("test".to_string());
        let service_not_found = Error::ServiceNotFound;
        let processor_not_found = Error::ProcessorNotFound;

        assert_eq!(empty_msg_id.to_string(), "Invalid message.");
        assert_eq!(invalid_addr.to_string(), "Invalid address.");
        assert_eq!(
            wrong_strategy.to_string(),
            "Wrong balancing strategy."
        );
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

        let err: Error = data
            .lock()
            .unwrap_err()
            .into();

        assert!(matches!(err, Error::LockFailed(_)))
    }

    #[test]
    fn queue_full_error() -> Result<()> {
        let src = Address::new("http://source.com")?;
        let dst = "http://destination.com";
        let payload = Bytes::default();

        let msg = Envelope::new(src.clone(), dst, payload.clone());
        let queue_full = Error::QueueFull(msg);
        assert!(matches!(queue_full, Error::QueueFull(_)));
        assert_eq!(
            queue_full.to_string(),
            format!(
                "Queue is full: {:?}",
                Envelope::new(src, dst, payload)
            )
        );
        Ok(())
    }
}
