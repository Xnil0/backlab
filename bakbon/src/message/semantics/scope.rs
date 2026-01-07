#[derive(Debug, PartialEq, Eq)]
pub enum Scope {
    Unicast,
    Multicast,
    Broadcast,
}

impl Default for Scope {
    fn default() -> Self { Self::Unicast }
}

impl From<String> for Scope {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "unicast" => Self::Unicast,
            "multicast" => Self::Multicast,
            "broadcast" => Self::Broadcast,
            _ => Self::Unicast,
        }
    }
}

impl From<Scope> for String {
    fn from(value: Scope) -> Self {
        match value {
            Scope::Unicast => "unicast",
            Scope::Multicast => "multicast",
            Scope::Broadcast => "broadcast",
        }
        .to_string()
    }
}
