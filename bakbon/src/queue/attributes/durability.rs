pub(in crate::queue) enum Durability {
    Memory,
    Disk,
    Replicated,
}

impl Default for Durability {
    fn default() -> Self { Self::Memory }
}

impl From<&str> for Durability {
    fn from(value: &str) -> Self {
        match value {
            "memory" => Self::Memory,
            "disk" => Self::Disk,
            "replicated" => Self::Replicated,
            _ => Self::default(),
        }
    }
}
