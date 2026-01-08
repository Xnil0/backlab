use std::fmt;

pub type MyResult<T> = Result<T, MyErr>;

pub enum MyErr {
    EmptyMessageId,
}

impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMessageId => f.write_str("Empty Message ID."),
        }
    }
}
