#![allow(unused)]

use {
    crate::{
        Address,
        Envelope,
        MyResult,
        protocol::Protocol,
    },
    bytes::Bytes,
};

pub struct GatewayBuilder {
    address:          Address,
    port:             u16,
    max_payload_size: Option<usize>,
    compression:      bool,
}

impl GatewayBuilder {
    fn new(address: impl Into<String>, port: u16) -> MyResult<Self> {
        Ok(Self {
            address: address.into().try_into()?,
            port,
            max_payload_size: None,
            compression: false,
        })
    }

    pub fn max_payload_size(mut self, size: usize) -> Self {
        self.max_payload_size = Some(size);
        self
    }

    pub fn enable_compression(mut self) -> Self {
        self.compression = true;
        self
    }

    pub fn build(self) -> Gateway {
        Gateway {
            address:          self.address,
            port:             self.port,
            max_payload_size: self.max_payload_size,
            compression:      self.compression,
        }
    }
}

pub struct Gateway {
    address:          Address,
    port:             u16,
    max_payload_size: Option<usize>,
    compression:      bool,
}

impl Gateway {
    pub fn builder(address: impl Into<String>, port: u16) -> MyResult<GatewayBuilder> {
        GatewayBuilder::new(address, port)
    }

    // pub fn handle(&self, data: Bytes) -> MyResult<Envelope> {}
}
