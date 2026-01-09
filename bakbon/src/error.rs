use std::fmt;

pub type MyResult<T> = Result<T, MyErr>;

#[derive(Debug)]
pub enum MyErr {
    EmptyMessageId,
    SendFailed,
    ReceptionFailed,
}

impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMessageId => f.write_str("Empty Message ID."),
            Self::SendFailed => f.write_str("Failed to send message."),
            Self::ReceptionFailed => f.write_str("Failed to receive message."),
        }
    }
}
