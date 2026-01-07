#[derive(Debug, PartialEq, Eq)]
pub enum DeliveryMode {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}

impl Default for DeliveryMode {
    fn default() -> Self { Self::AtLeastOnce }
}
