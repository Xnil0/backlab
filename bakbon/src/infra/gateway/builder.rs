use crate::{
    Gateway,
    MyResult,
    core::{
        Address,
        Protocol,
    },
};

pub struct GatewayBuilder {
    address:          Address,
    port:             u16,
    protocol:         Protocol,
    max_payload_size: Option<usize>,
    compression:      bool,
}

impl GatewayBuilder {
    pub(super) fn new(address: &str, port: u16) -> MyResult<Self> {
        Ok(Self {
            address: address.try_into()?,
            port,
            protocol: Protocol::default(),
            max_payload_size: None,
            compression: false,
        })
    }

    pub fn protocol(mut self, protocol: &str) -> Self {
        self.protocol = Protocol::from(protocol);
        self
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
            protocol:         self.protocol,
            max_payload_size: self.max_payload_size,
            compression:      self.compression,
        }
    }
}
