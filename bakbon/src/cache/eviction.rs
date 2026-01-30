#[derive(Debug, Clone, Copy)]
pub enum Eviction {
    Fifo, // First In First Out
    Lru,  // Least Recently Used
    Lfu,  // Least Frequently Used
}

impl Default for Eviction {
    fn default() -> Self { Self::Lru }
}

impl From<&str> for Eviction {
    fn from(value: &str) -> Self {
        match value {
            "FIFO" => Self::Fifo,
            "LRU" => Self::Lru,
            "LFU" => Self::Lfu,
            _ => Self::default(),
        }
    }
}

impl AsRef<str> for Eviction {
    fn as_ref(&self) -> &str {
        match self {
            Self::Fifo => "FIFO",
            Self::Lru => "LRU",
            Self::Lfu => "LFU",
        }
    }
}
