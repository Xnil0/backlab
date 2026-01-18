use bakbon::{
    Address,
    Envelope,
    MyErr,
    MyResult,
    ProcMap,
    Processor,
    Reply,
    Service,
};

pub struct EchoProc;

impl Processor for EchoProc {
    fn execute(&self, msg: Envelope) -> MyResult<Reply> {
        let src = Address::new(msg.destination()).unwrap();
        let dst = msg.source().to_string();
        let payload = msg.payload().clone();

        let mut reply = Envelope::new(src, dst, payload);
        reply.copy_headers(msg);

        Ok(Some(reply))
    }
}

pub struct EchoService {
    address:    Address,
    processors: ProcMap,
}

impl EchoService {
    pub fn new(address: Address) -> Self {
        let mut processors = ProcMap::new();
        processors.insert(String::from("/echo"), Box::new(EchoProc));

        Self {
            address,
            processors,
        }
    }
}

impl Service for EchoService {
    fn address(&self) -> &Address { &self.address }

    fn duplicate(&self) -> Box<dyn Service> { Box::new(Self::new(self.address.clone())) }

    fn process(&self, message: Envelope) -> MyResult<Reply> {
        let path = self.address.path();
        match self.processors.get(path) {
            Some(processor) => processor.execute(message),
            None => Err(MyErr::ProcessorNotFound),
        }
    }
}
