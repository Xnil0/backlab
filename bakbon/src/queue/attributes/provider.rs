#[derive(Debug, PartialEq, Eq)]
pub(in crate::queue) enum QueueProvider {
    Memory,
    Kafka,
    RabbitMq,
    Redis,
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
            "redis" => Self::Redis,
            provider => Self::Custom(provider.to_string()),
        }
    }
}

impl AsRef<str> for QueueProvider {
    fn as_ref(&self) -> &str {
        match self {
            Self::Memory => "memory",
            Self::Kafka => "kafka",
            Self::RabbitMq => "rabbitmq",
            Self::Redis => "redis",
            Self::Custom(provider) => provider,
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
    fn default_queue_provider() {
        let provider = QueueProvider::default();
        assert_eq!(provider.as_ref(), "memory");
        assert_eq!(provider, QueueProvider::Memory);
    }

    #[test]
    fn queue_provider_from_str() {
        let provider_str = "kafka";
        let provider = QueueProvider::from(provider_str);
        assert_eq!(provider.as_ref(), "kafka");
        assert_eq!(provider, QueueProvider::Kafka);
    }

    #[test]
    fn str_into_queue_provider() {
        let provider_str = "rabbitmq";
        let provider: QueueProvider = provider_str.into();
        assert_eq!(provider.as_ref(), provider_str);
        assert_eq!(provider, QueueProvider::RabbitMq);
    }

    #[test]
    fn redis_queue_provider() {
        let provider_str = "redis";
        let provider = QueueProvider::from(provider_str);
        assert_eq!(provider.as_ref(), provider_str);
        assert_eq!(provider, QueueProvider::Redis);
    }

    #[test]
    fn custom_queue_provider() {
        let provider_str = "aws";
        let provider = QueueProvider::from(provider_str);
        assert_eq!(provider.as_ref(), provider_str);
        assert_eq!(
            provider,
            QueueProvider::Custom(provider_str.to_string())
        );
    }
}
