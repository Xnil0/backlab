use {
    super::{
        Endpoint,
        Envelope,
        MyResult,
        Reply,
    },
    std::collections::HashMap,
};

pub trait Processor {
    fn process(&self, message: Envelope) -> MyResult<Reply>;
}

pub type ProcMap = HashMap<String, Box<dyn Processor>>;

pub trait Service {
    fn endpoint(&self) -> &Endpoint;
    fn dispatch(&self, message: Envelope) -> MyResult<Reply>;
}
