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
        let destination = format!("{}://{}", self.protocol, path);

        Envelope::new(
            self.address.to_string(),
            destination,
            payload,
        )
    }

    pub fn address(&self) -> String { self.address.to_string() }
}
