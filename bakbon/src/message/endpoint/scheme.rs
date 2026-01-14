use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Scheme {
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

impl From<&str> for Scheme {
    fn from(value: &str) -> Self {
        match value {
            "tcp" => Scheme::Tcp,
            "udp" => Scheme::Udp,
            "http" => Scheme::Http,
            "https" => Scheme::Https,
            "grpc" => Scheme::Grpc,
            "mqtt" => Scheme::Mqtt,
            "serial" => Scheme::Serial,
            "inproc" => Scheme::InProc,
            _ => Scheme::Custom(value.to_string()),
        }
    }
}

impl fmt::Display for Scheme {
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
