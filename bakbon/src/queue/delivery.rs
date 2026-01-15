pub(super) enum DeliveryGuarantee {
    AtLeastOnce,
    AtMostOnce,
    ExactlyOnce,
}

impl Default for DeliveryGuarantee {
    fn default() -> Self { Self::AtLeastOnce }
}

impl From<&str> for DeliveryGuarantee {
    fn from(value: &str) -> Self {
        match value {
            "at_least_once" => Self::AtLeastOnce,
            "at_most_once" => Self::AtMostOnce,
            "exactly_once" => Self::ExactlyOnce,
            _ => Self::default(),
        }
    }
}
