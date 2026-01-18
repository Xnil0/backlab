mod builder;

use {
    crate::{
        Envelope,
        MyResult,
        core::{
            Address,
            Protocol,
        },
    },
    builder::GatewayBuilder,
    bytes::Bytes,
};

pub struct Gateway {
    address:          Address,
    port:             u16,
    protocol:         Protocol,
    max_payload_size: Option<usize>,
    compression:      bool,
}

impl Gateway {
    pub fn builder(address: &str, port: u16) -> MyResult<GatewayBuilder> {
        GatewayBuilder::new(address, port)
    }

    pub fn handle(&self, path: &str, payload: Bytes) -> Envelope {
        let destination = format!("{}:/{}", self.protocol, path);

        Envelope::new(
            self.address.to_string(),
            destination,
            payload,
        )
    }

    pub fn address(&self) -> String { self.address.to_string() }

    pub fn port(&self) -> u16 { self.port }

    pub fn protocol(&self) -> &str { self.protocol.as_ref() }

    pub fn max_payload_size(&self) -> Option<usize> { self.max_payload_size }

    pub fn compression(&self) -> bool { self.compression }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_gateway() -> MyResult<()> {
        let address = "https://gateway.com";
        let port = 8080;
        let payload_size = 1024;

        let gateway = Gateway::builder(address, port)?
            .protocol("grpc")
            .max_payload_size(payload_size)
            .enable_compression()
            .build();

        assert!(gateway.compression());
        assert_eq!(gateway.address(), address);
        assert_eq!(gateway.port(), port);
        assert_eq!(gateway.protocol(), "grpc");
        assert_eq!(
            gateway.max_payload_size(),
            Some(payload_size)
        );
        Ok(())
    }

    #[test]
    fn gateway_handle() -> MyResult<()> {
        let path = "/api/v1/users";
        let payload = Bytes::from("Hello, World!");

        let gateway = Gateway::builder("https://gateway.com", 8080)?
            .protocol("grpc")
            .max_payload_size(1024)
            .enable_compression()
            .build();

        let envelope = gateway.handle(path, payload.clone());
        assert_eq!(envelope.source(), "https://gateway.com");
        assert_eq!(envelope.destination(), "grpc://api/v1/users");
        assert_eq!(envelope.payload(), &payload);
        Ok(())
    }
}
