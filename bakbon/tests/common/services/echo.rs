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
    fn execute(&self, message: Envelope) -> MyResult<Reply> {
        let payload = message.payload().clone();
        let reply = message.reply(payload);
        Ok(Some(reply))
    }
}

pub struct EchoService {
    address:   Address,
    processors: ProcMap,
}

impl EchoService {
    pub fn new(address: impl ToString) -> MyResult<Self> {
        let address = Address::try_from(address.to_string())?;
        let mut processors = ProcMap::new();
        processors.insert(String::from(""), Box::new(EchoProc));

        Ok(Self {
            address,
            processors,
        })
    }
}

impl Service for EchoService {
    fn address(&self) -> &Address { &self.address }

    fn process(&self, message: Envelope) -> MyResult<Reply> {
        let path = self.address().path();
        match self.processors.get(path) {
            Some(processor) => processor.execute(message),
            None => Err(MyErr::ProcessorNotFound),
        }
    }
}
