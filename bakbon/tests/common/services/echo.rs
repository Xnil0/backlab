use bakbon::{
    Address,
    Envelope,
    Error,
    ProcMap,
    Processor,
    Reply,
    Result,
    Service,
    ServiceBox,
};

#[derive(Debug)]
pub struct EchoProc;

impl Processor for EchoProc {
    fn execute(&self, msg: Envelope) -> Result<Reply> {
        let payload = msg.payload().clone();
        let reply = msg.into_reply(payload);
        Ok(Some(reply))
    }
}

#[derive(Debug)]
pub struct EchoService {
    address:    Address,
    processors: ProcMap,
}

impl EchoService {
    pub fn new(address: Address) -> Self {
        let mut processors = ProcMap::new();
        processors.insert(String::from(""), Box::new(EchoProc));

        Self {
            address,
            processors,
        }
    }
}

impl Service for EchoService {
    fn address(&self) -> &Address { &self.address }

    fn duplicate(&self) -> ServiceBox {
        let address = self.address.clone();
        let dupe_service = Self::new(address);
        Box::new(dupe_service)
    }

    fn process(&self, message: Envelope) -> Result<Reply> {
        let path = self.address.path();
        match self.processors.get(path) {
            Some(processor) => processor.execute(message),
            None => Err(Error::ProcessorNotFound),
        }
    }
}
