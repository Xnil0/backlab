#[derive(Debug, PartialEq, Eq)]
pub(in crate::queue) enum Ordering {
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

impl AsRef<str> for Ordering {
    fn as_ref(&self) -> &str {
        match self {
            Self::Fifo => "fifo",
            Self::Priority => "priority",
            Self::Unordered => "unordered",
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
    fn default_ordering() {
        let ordering = Ordering::default();
        assert_eq!(ordering, Ordering::Fifo);
        assert_eq!(ordering.as_ref(), "fifo");
    }

    #[test]
    fn ordering_from_str() {
        let ordering_str = "priority";
        let ordering = Ordering::from(ordering_str);
        assert_eq!(ordering, Ordering::Priority);
        assert_eq!(ordering.as_ref(), ordering_str);
    }

    #[test]
    fn str_into_ordering() {
        let ordering_str = "unordered";
        let ordering: Ordering = ordering_str.into();
        assert_eq!(ordering, Ordering::Unordered);
        assert_eq!(ordering.as_ref(), ordering_str);
    }

    #[test]
    fn default_ordering_from_invalid_str() {
        let ordering_str = "invalid_ordering";
        let ordering = Ordering::from(ordering_str);
        assert_eq!(ordering, Ordering::default());
        assert_eq!(ordering.as_ref(), "fifo");
    }
}
