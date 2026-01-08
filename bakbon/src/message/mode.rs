#[derive(Debug, PartialEq, Eq)]
pub enum Guarantee {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}

impl Default for Guarantee {
    fn default() -> Self { Self::AtLeastOnce }
}
