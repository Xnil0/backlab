use {
    crate::{
        Address,
        Envelope,
        Protocol,
        Result,
    },
    bytes::Bytes,
};

/// `Gateway` is a network endpoint that can send messages to other
/// [`Service`](crate::Service)s.
///
/// It provides methods to build a new `Gateway`, handle incoming messages,
/// and retrieve information about the `Gateway`.
///
/// # Examples
///
/// ```rust
/// use {
///     bakbon::*,
///     bytes::Bytes,
/// };
///
/// pub struct BasicGateway {
///     address:          Address,
///     protocol:         Protocol,
/// }
///
/// impl Gateway for BasicGateway {
///     fn address(&self) -> &Address {
///         &self.address
///     }
///
///     fn protocol(&self) -> &Protocol {
///         &self.protocol
///     }
///
///     fn handle(&self, path: &str, data: Bytes) -> Result<Envelope> {
///         let destination = Address::parse(format!("{}:/{}", self.protocol, path))?;
///         let message = Envelope::new(self.address().clone(), destination, data);
///         Ok(message)
///     }
/// }
///
/// let address = Address::parse("https://gateway.com");
/// assert!(address.is_ok());
/// let address = address.unwrap();
///
/// let gateway = BasicGateway {
///     address:          address.clone(),
///     protocol:         Protocol::Grpc,
/// };
///
/// let data = Bytes::from("Hello, World!");
/// let msg = gateway.handle("/api/v1/data", data.clone());
/// assert!(msg.is_ok());
///
/// let msg = msg.unwrap();
/// assert_eq!(msg.payload(), &data);
///
/// let dst = Address::parse("grpc://api/v1/data");
/// assert!(dst.is_ok());
/// assert_eq!(msg.destination(), &dst.unwrap());
///
/// assert_eq!(gateway.address(), &address);
/// assert_eq!(gateway.protocol().as_ref(), "grpc");
/// ```
pub trait Gateway {
    fn address(&self) -> &Address;
    fn protocol(&self) -> &Protocol;
    fn handle(&self, path: &str, data: Bytes) -> Result<Envelope>;
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Payload,
    };

    const URI: &str = "https://gateway.com";

    pub struct BasicGateway {
        address:  Address,
        protocol: Protocol,
    }

    impl BasicGateway {
        pub fn new(addr: &str, proto: &str) -> Result<Self> {
            Ok(Self {
                address:  Address::parse(addr)?,
                protocol: proto.into(),
            })
        }
    }

    impl Gateway for BasicGateway {
        fn address(&self) -> &Address { &self.address }

        fn protocol(&self) -> &Protocol { &self.protocol }

        fn handle(&self, path: &str, data: Bytes) -> Result<Envelope> {
            let destination = Address::parse(format!("{}:/{}", self.protocol, path))?;
            let message = Envelope::new(self.address().clone(), destination, data);
            Ok(message)
        }
    }

    #[test]
    fn basic_gateway() -> Result<()> {
        let gateway = BasicGateway::new(URI, "grpc")?;
        assert_eq!(gateway.address().to_string(), URI);
        assert_eq!(gateway.protocol().as_ref(), "grpc");
        Ok(())
    }

    #[test]
    fn gateway_handle() -> Result<()> {
        let path = "/users";
        let data = Payload::from("Hello, World!");

        let gateway = BasicGateway::new(URI, "http")?;

        let msg = gateway.handle(path, data.clone())?;
        assert_eq!(msg.source().to_string(), URI);
        assert_eq!(msg.destination().to_string(), "http://users");
        assert_eq!(msg.payload(), &data);
        Ok(())
    }
}
