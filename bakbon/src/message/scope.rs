#[derive(Debug, PartialEq, Eq)]
pub enum Fanout {
    Unicast,
    Multicast,
    Broadcast,
}

impl Default for Fanout {
    fn default() -> Self { Self::Unicast }
}

impl From<String> for Fanout {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "unicast" => Self::Unicast,
            "multicast" => Self::Multicast,
            "broadcast" => Self::Broadcast,
            _ => Self::Unicast,
        }
    }
}

impl From<Fanout> for String {
    fn from(value: Fanout) -> Self {
        match value {
            Fanout::Unicast => "unicast",
            Fanout::Multicast => "multicast",
            Fanout::Broadcast => "broadcast",
        }
        .to_string()
    }
}
