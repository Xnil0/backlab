use {
    bakbon::{
        Address,
        Envelope,
        Gateway,
        Protocol,
        Result,
    },
    bytes::Bytes,
};

pub struct ApiGateway {
    address:  Address,
    protocol: Protocol,
}

impl ApiGateway {
    pub fn new(uri: &str, protocol: &str) -> Result<Self> {
        Ok(Self {
            address:  Address::parse(uri)?,
            protocol: protocol.into(),
        })
    }
}

impl Gateway for ApiGateway {
    fn address(&self) -> &Address { &self.address }

    fn protocol(&self) -> &Protocol { &self.protocol }

    fn handle(&self, path: &str, data: Bytes) -> Result<Envelope> {
        let destination = Address::parse(format!("{}:/{}", self.protocol, path))?;
        let message = Envelope::new(self.address().clone(), destination, data);
        Ok(message)
    }
}
