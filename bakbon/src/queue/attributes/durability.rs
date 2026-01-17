#[derive(Debug, PartialEq, Eq)]
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

impl AsRef<str> for Durability {
    fn as_ref(&self) -> &str {
        match self {
            Self::Memory => "memory",
            Self::Disk => "disk",
            Self::Replicated => "replicated",
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
    fn default_durability() {
        let durability = Durability::default();
        assert_eq!(durability.as_ref(), "memory");
        assert_eq!(durability, Durability::Memory);
    }

    #[test]
    fn durability_from_str() {
        let durability_str = "disk";
        let durability = Durability::from(durability_str);
        assert_eq!(durability.as_ref(), durability_str);
        assert_eq!(durability, Durability::Disk);
    }

    #[test]
    fn str_into_durability() {
        let durability_str = "replicated";
        let durability: Durability = durability_str.into();
        assert_eq!(durability.as_ref(), durability_str);
        assert_eq!(durability, Durability::Replicated);
    }

    #[test]
    fn default_durability_from_invalid_str() {
        let durability_str = "invalid_durability";
        let durability = Durability::from(durability_str);
        assert_eq!(durability.as_ref(), "memory");
        assert_eq!(durability, Durability::Memory);
    }
}
