#[derive(Debug, PartialEq, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
}

impl Default for Priority {
    fn default() -> Self { Priority::Normal }
}
