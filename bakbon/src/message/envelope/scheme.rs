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

impl Scheme {
    pub fn as_str(&self) -> &str {
        match self {
            Scheme::Tcp => "tcp",
            Scheme::Udp => "udp",
            Scheme::Http => "http",
            Scheme::Https => "https",
            Scheme::Grpc => "grpc",
            Scheme::Serial => "serial",
            Scheme::InProc => "inproc",
            Scheme::Custom(s) => s,
        }
    }
}
