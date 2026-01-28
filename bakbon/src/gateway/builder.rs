use crate::{
    Gateway,
    Result,
    core::{
        Address,
        Protocol,
    },
};

/// Builder for creating a new `Gateway`.
///
/// The `GatewayBuilder` provides a convenient way to create a new
/// `Gateway` instance with custom configuration using methods chain.
///
/// # Examples
///
/// ```rust
/// use bakbon::*;
///
/// let gw_builder = Gateway::builder("https://gateway.com", 8080);
/// assert!(gw_builder.is_ok());
/// let gw_builder = gw_builder.unwrap();
///
/// let gw_builder = gw_builder.protocol("https");
///
/// let gateway = gw_builder.build();
/// assert_ne!(gateway.protocol(), &Protocol::default());
/// assert_eq!(gateway.protocol(), &Protocol::Http { secure: true });
/// ```
pub struct GatewayBuilder {
    address:          Address,
    port:             u16,
    protocol:         Protocol,
    max_payload_size: Option<usize>,
    compression:      bool,
}

impl GatewayBuilder {
    pub(super) fn new(address: &str, port: u16) -> Result<Self> {
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

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use super::*;

    const ADDRESS: &str = "https://gateway.com";
    const PORT: u16 = 8080;

    #[test]
    fn default_gateway_builder() -> Result<()> {
        let builder = GatewayBuilder::new(ADDRESS, PORT)?;
        assert_eq!(builder.address.to_string(), ADDRESS);
        assert_eq!(builder.port, PORT);
        assert_eq!(builder.protocol, Protocol::default());
        assert_eq!(builder.max_payload_size, None);
        assert_eq!(builder.compression, false);
        Ok(())
    }

    #[test]
    fn build_default_gateway() -> Result<()> {
        let builder = GatewayBuilder::new(ADDRESS, PORT)?;
        let gateway = builder.build();
        assert_eq!(gateway.address().to_string(), ADDRESS);
        assert_eq!(gateway.port(), PORT);
        assert_eq!(gateway.protocol(), &Protocol::InProc);
        assert_eq!(gateway.max_payload_size, None);
        assert_eq!(gateway.compression, false);
        Ok(())
    }

    #[test]
    fn build_gateway_with_protocol() -> Result<()> {
        let builder = GatewayBuilder::new(ADDRESS, PORT)?;
        let protocol = "http";
        let gateway = builder
            .protocol(protocol)
            .build();
        assert_eq!(
            gateway.protocol(),
            &Protocol::Http {
                secure: false,
            }
        );
        Ok(())
    }

    #[test]
    fn build_gateway_with_max_payload_size() -> Result<()> {
        let builder = GatewayBuilder::new(ADDRESS, PORT)?;
        let max_payload_size = 1024;

        let gateway = builder
            .max_payload_size(max_payload_size)
            .build();

        assert_eq!(
            gateway.max_payload_size(),
            Some(max_payload_size)
        );
        Ok(())
    }

    #[test]
    fn build_gateway_with_compression() -> Result<()> {
        let builder = GatewayBuilder::new(ADDRESS, PORT)?;
        let gateway = builder
            .enable_compression()
            .build();
        assert_eq!(gateway.compression(), true);
        Ok(())
    }
}
