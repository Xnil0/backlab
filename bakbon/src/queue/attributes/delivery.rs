#[derive(Debug, PartialEq, Eq)]
pub(in crate::queue) enum DeliveryGuarantee {
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

impl AsRef<str> for DeliveryGuarantee {
    fn as_ref(&self) -> &str {
        match self {
            Self::AtLeastOnce => "at_least_once",
            Self::AtMostOnce => "at_most_once",
            Self::ExactlyOnce => "exactly_once",
        }
    }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_delivery_guarantee() {
        let guarantee = DeliveryGuarantee::default();
        assert_eq!(guarantee, DeliveryGuarantee::AtLeastOnce);
        assert_eq!(guarantee.as_ref(), "at_least_once");
    }

    #[test]
    fn delivery_guarantee_from_str() {
        let guarantee_str = "at_most_once";
        let guarantee = DeliveryGuarantee::from(guarantee_str);
        assert_eq!(guarantee, DeliveryGuarantee::AtMostOnce);
        assert_eq!(guarantee.as_ref(), guarantee_str);
    }

    #[test]
    fn str_into_delivery_guarantee() {
        let guarantee_str = "exactly_once";
        let guarantee: DeliveryGuarantee = guarantee_str.into();
        assert_eq!(guarantee, DeliveryGuarantee::ExactlyOnce);
        assert_eq!(guarantee.as_ref(), guarantee_str);
    }

    #[test]
    fn default_delivery_guarante_from_invalid_str() {
        let guarantee_str = "invalid_delivery_guarantee";
        let guarantee = DeliveryGuarantee::from(guarantee_str);
        assert_eq!(guarantee, DeliveryGuarantee::default());
        assert_eq!(guarantee.as_ref(), "at_least_once");
    }
}
