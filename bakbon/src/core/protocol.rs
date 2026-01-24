use std::fmt;

/// Transport protocol used in BakBon addresses and gateways.
///
/// Models how messages are transmitted between nodes, via for
/// example `TCP` (Transmission Control Protocol), `HTTP` (HyperText
/// Transfer Protocol) `GRPC` (Google Remote Procedure Call), `MQTT`
/// (Message Queuing Telemetry Transport), and more...
///
/// Appears in [`Address`](super::Address) as the `URI` scheme and in
/// [`Gateway`](crate::Gateway) to define how external requests are turned
/// into internal messages.
///
/// The `Http` variant switches between http and https protocol via the
/// `secure` flag and the `Custom` variant lets support and define any
/// additional protocol.
///
/// ## Defaults
///
/// The default `Protocol` variant is `InProc`, which is suitable for
/// in-process testing and local implementations.
///
/// # Examples
///
/// ```
/// use bakbon::Protocol;
///
/// let protocol = Protocol::Http { secure: true };
/// assert_eq!(protocol.as_ref(), "https");
///
/// let protocol = Protocol::Custom("myproto".to_string());
/// assert_eq!(protocol.as_ref(), "myproto");
///
/// let protocol = Protocol::from("http");
/// assert_eq!(protocol.as_ref(), "http");
///
/// if let Protocol::Http { secure } = protocol {
///     assert!(!secure);
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Protocol {
    Tcp,
    Udp,
    Http { secure: bool },
    Grpc,
    Mqtt,
    Serial,
    InProc,
    Custom(String),
}

impl Default for Protocol {
    fn default() -> Self { Self::InProc }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tcp => write!(f, "tcp"),
            Self::Udp => write!(f, "udp"),
            Self::Http { secure } if *secure => write!(f, "https"),
            Self::Http { .. } => write!(f, "http"),
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
            "grpc" => Self::Grpc,
            "mqtt" => Self::Mqtt,
            "serial" => Self::Serial,
            "inproc" => Self::InProc,
            "http" => Self::Http {
                secure: false,
            },
            "https" => Self::Http {
                secure: true,
            },
            _ => Self::Custom(value.to_string()),
        }
    }
}

impl AsRef<str> for Protocol {
    fn as_ref(&self) -> &str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Http { secure } if *secure => "https",
            Self::Http { .. } => "http",
            Self::Grpc => "grpc",
            Self::Mqtt => "mqtt",
            Self::Serial => "serial",
            Self::InProc => "inproc",
            Self::Custom(s) => s,
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
    fn default_protocol() {
        let proto = Protocol::default();
        assert_eq!(proto, Protocol::InProc)
    }

    #[test]
    fn protocol_from_str() {
        let proto_str = "inproc";
        let proto = Protocol::from(proto_str);
        assert_eq!(proto, Protocol::InProc)
    }

    #[test]
    fn str_into_protocol() {
        let proto_str = "inproc";
        let proto: Protocol = proto_str.into();
        assert_eq!(proto, Protocol::InProc);
    }

    #[test]
    fn protocol_reference() {
        let proto = Protocol::default();
        let proto_ref = proto.as_ref();
        assert_eq!(proto_ref, "inproc");
    }

    #[test]
    fn format_protocol() {
        let proto = Protocol::default();
        let fmt_proto = format!("{proto}");
        assert_eq!(fmt_proto, "inproc")
    }

    #[test]
    fn tcp_protocol() {
        let proto_str = "tcp";
        let proto = Protocol::from(proto_str);
        let fmt_proto = format!("{}", proto);

        assert_eq!(proto, Protocol::Tcp);
        assert_eq!(proto.as_ref(), proto_str);
        assert_eq!(fmt_proto, proto_str);
        assert_eq!(proto.to_string(), fmt_proto);
    }

    #[test]
    fn udp_protocol() {
        let proto_str = "udp";
        let proto = Protocol::from(proto_str);
        let fmt_proto = format!("{}", proto);

        assert_eq!(proto, Protocol::Udp);
        assert_eq!(proto.as_ref(), proto_str);
        assert_eq!(fmt_proto, proto_str);
        assert_eq!(proto.to_string(), fmt_proto);
    }

    #[test]
    fn http_protocol() {
        let proto_str = "http";
        let proto = Protocol::from(proto_str);
        let fmt_proto = format!("{}", proto);

        assert_eq!(
            proto,
            Protocol::Http {
                secure: false,
            }
        );
        assert_eq!(proto.as_ref(), proto_str);
        assert_eq!(fmt_proto, proto_str);
        assert_eq!(proto.to_string(), fmt_proto);
    }

    #[test]
    fn https_protocol() {
        let proto_str = "https";
        let proto = Protocol::from(proto_str);
        let fmt_proto = format!("{}", proto);

        assert_eq!(
            proto,
            Protocol::Http {
                secure: true,
            }
        );
        assert_eq!(proto.as_ref(), proto_str);
        assert_eq!(fmt_proto, proto_str);
        assert_eq!(proto.to_string(), fmt_proto);
    }

    #[test]
    fn grpc_protocol() {
        let proto_str = "grpc";
        let proto = Protocol::from(proto_str);
        let fmt_proto = format!("{}", proto);

        assert_eq!(proto, Protocol::Grpc);
        assert_eq!(proto.as_ref(), proto_str);
        assert_eq!(fmt_proto, proto_str);
        assert_eq!(proto.to_string(), fmt_proto);
    }

    #[test]
    fn mqtt_protocol() {
        let proto_str = "mqtt";
        let proto = Protocol::from(proto_str);
        let fmt_proto = format!("{}", proto);

        assert_eq!(proto, Protocol::Mqtt);
        assert_eq!(proto.as_ref(), proto_str);
        assert_eq!(fmt_proto, proto_str);
        assert_eq!(proto.to_string(), fmt_proto);
    }

    #[test]
    fn serial_protocol() {
        let proto_str = "serial";
        let proto = Protocol::from(proto_str);
        let fmt_proto = format!("{}", proto);

        assert_eq!(proto, Protocol::Serial);
        assert_eq!(proto.as_ref(), proto_str);
        assert_eq!(fmt_proto, proto_str);
        assert_eq!(proto.to_string(), fmt_proto);
    }

    #[test]
    fn custom_protocol() {
        let proto_str = "mpsc";
        let proto = Protocol::from(proto_str);
        let fmt_proto = format!("{}", proto);

        assert_eq!(proto.as_ref(), proto_str);
        assert_eq!(fmt_proto, proto_str);
        assert_eq!(
            proto,
            Protocol::Custom(proto_str.to_string())
        );
    }
}
