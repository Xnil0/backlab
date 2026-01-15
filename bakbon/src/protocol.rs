use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Http,
    Https,
    Grpc,
    Mqtt,
    Serial,
    InProc,
    Custom(String),
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tcp => write!(f, "tcp"),
            Self::Udp => write!(f, "udp"),
            Self::Http => write!(f, "http"),
            Self::Https => write!(f, "https"),
            Self::Grpc => write!(f, "grpc"),
            Self::Mqtt => write!(f, "mqtt"),
            Self::Serial => write!(f, "serial"),
            Self::InProc => write!(f, "inproc"),
            Self::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl From<&str> for Protocol {
    fn from(value: &str) -> Self {
        match value {
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            "http" => Self::Http,
            "https" => Self::Https,
            "grpc" => Self::Grpc,
            "mqtt" => Self::Mqtt,
            "serial" => Self::Serial,
            "inproc" => Self::InProc,
            _ => Self::Custom(value.to_string()),
        }
    }
}
