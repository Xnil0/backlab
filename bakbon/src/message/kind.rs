#[derive(Debug, PartialEq, Eq)]
pub enum MessageKind {
    Command,
    Query,
    Reply,
    Event,
    Telemetry,
    Custom(String),
}

impl From<String> for MessageKind {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "command" => Self::Command,
            "query" => Self::Query,
            "reply" => Self::Reply,
            "event" => Self::Event,
            "telemetry" => Self::Telemetry,
            _ => Self::Custom(value),
        }
    }
}

impl AsRef<str> for MessageKind {
    fn as_ref(&self) -> &str {
        match self {
            MessageKind::Command => "command",
            MessageKind::Query => "query",
            MessageKind::Reply => "reply",
            MessageKind::Event => "event",
            MessageKind::Telemetry => "telemetry",
            MessageKind::Custom(kind) => kind.as_str(),
        }
    }
}
