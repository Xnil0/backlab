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

impl From<MessageKind> for String {
    fn from(value: MessageKind) -> Self {
        match value {
            MessageKind::Command => "command",
            MessageKind::Query => "query",
            MessageKind::Reply => "reply",
            MessageKind::Event => "event",
            MessageKind::Telemetry => "telemetry",
            MessageKind::Custom(ref kind) => kind,
        }
        .to_string()
    }
}
