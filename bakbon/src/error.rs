use std::fmt;

pub type MyResult<T> = Result<T, MyErr>;

#[derive(Debug)]
pub enum MyErr {
    EmptyMessageId,
    InvalidEndpoint,
    SendFailed,
    ReceptionFailed,
    NoProcessor,
}

impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMessageId => f.write_str("Empty Message ID."),
            Self::InvalidEndpoint => f.write_str("Invalid endpoint."),
            Self::SendFailed => f.write_str("Failed to send message."),
            Self::ReceptionFailed => f.write_str("Failed to receive message."),
            Self::NoProcessor => f.write_str("No processor found."),
        }
    }
}
