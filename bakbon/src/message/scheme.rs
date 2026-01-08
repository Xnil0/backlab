pub enum Scheme {
    Tcp,
    Udp,
    Http,
    Https,
    Grpc,
    Serial,
    InProc,
    Custom(String),
}

impl From<&str> for Scheme {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            "http" => Self::Http,
            "https" => Self::Https,
            "grpc" => Self::Grpc,
            "serial" => Self::Serial,
            "inproc" => Self::InProc,
            any => Self::Custom(any.to_string()),
        }
    }
}
