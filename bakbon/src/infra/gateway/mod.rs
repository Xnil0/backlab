mod builder;

use {
    crate::{
        Envelope,
        Payload,
        Result,
        core::{
            Address,
            Protocol,
        },
    },
    builder::GatewayBuilder,
};

pub struct Gateway {
    address:          Address,
    port:             u16,
    protocol:         Protocol,
    max_payload_size: Option<usize>,
    compression:      bool,
}

impl Gateway {
    pub fn builder(address: &str, port: u16) -> Result<GatewayBuilder> {
        GatewayBuilder::new(address, port)
    }

    pub fn handle(&self, path: &str, payload: Payload) -> Result<Envelope> {
        let dst_str = format!("{}:/{}", self.protocol, path);
        let destination = Address::parse(dst_str.as_str())?;

        let msg = Envelope::new(self.address.clone(), destination, payload);
        Ok(msg)
    }

    pub fn address(&self) -> &Address { &self.address }

    pub fn port(&self) -> u16 { self.port }

    pub fn protocol(&self) -> &Protocol { &self.protocol }

    pub fn max_payload_size(&self) -> Option<usize> { self.max_payload_size }

    pub fn compression(&self) -> bool { self.compression }
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

    #[test]
    fn build_gateway() -> Result<()> {
        let port = 8080;
        let payload_size = 1024;

        let gateway = Gateway::builder(URI, port)?
            .protocol("grpc")
            .max_payload_size(payload_size)
            .enable_compression()
            .build();

        assert!(gateway.compression());
        assert_eq!(gateway.address().to_string(), URI);
        assert_eq!(gateway.port(), port);
        assert_eq!(gateway.protocol(), &Protocol::Grpc);
        assert_eq!(
            gateway.max_payload_size(),
            Some(payload_size)
        );
        Ok(())
    }

    #[test]
    fn gateway_handle() -> Result<()> {
        let path = "/users";
        let payload = Payload::from("Hello, World!");

        let gateway = Gateway::builder(URI, 8080)?
            .protocol("grpc")
            .max_payload_size(1024)
            .enable_compression()
            .build();

        let msg = gateway.handle(path, payload.clone())?;
        assert_eq!(msg.source().to_string(), URI);
        assert_eq!(msg.destination().to_string(), "grpc://users");
        assert_eq!(msg.payload(), &payload);
        Ok(())
    }
}
