pub(super) enum Ordering {
    Fifo,
    Priority,
    Unordered,
}

impl Default for Ordering {
    fn default() -> Self { Self::Fifo }
}

impl From<&str> for Ordering {
    fn from(value: &str) -> Self {
        match value {
            "fifo" => Self::Fifo,
            "priority" => Self::Priority,
            "unordered" => Self::Unordered,
            _ => Self::default(),
        }
    }
}
