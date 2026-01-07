pub mod mode;
pub mod priority;

pub use {
    mode::DeliveryMode,
    priority::Priority,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct OrderingKey(String);

impl OrderingKey {
    pub fn new(key: impl Into<String>) -> Self { Self(key.into()) }

    pub fn value(&self) -> &str { &self.0 }
}
