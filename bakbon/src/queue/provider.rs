#[allow(unused)]
pub(super) enum QueueProvider {
    Memory,
    Kafka,
    RabbitMq,
    Custom(String),
}

impl Default for QueueProvider {
    fn default() -> Self { Self::Memory }
}

impl From<&str> for QueueProvider {
    fn from(value: &str) -> Self {
        match value {
            "memory" => Self::Memory,
            "kafka" => Self::Kafka,
            "rabbitmq" => Self::RabbitMq,
            provider => Self::Custom(provider.to_string()),
        }
    }
}
