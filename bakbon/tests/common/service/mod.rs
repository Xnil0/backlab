use bakbon::{
    Endpoint,
    Envelope,
    MyErr,
    MyResult,
    ProcMap,
    Processor,
    Reply,
    Service,
};

pub struct Echo;

impl Processor for Echo {
    fn process(&self, message: Envelope) -> MyResult<Reply> { Ok(Some(message)) }
}

pub struct EchoService {
    endpoint:   Endpoint,
    processors: ProcMap,
}

impl Service for EchoService {
    fn endpoint(&self) -> &Endpoint { &self.endpoint }

    fn dispatch(&self, message: Envelope) -> MyResult<Reply> {
        let path = self.endpoint().path();
        match self.processors.get(path) {
            Some(processor) => processor.process(message),
            None => Err(MyErr::NoProcessor),
        }
    }
}
