use {
    super::{
        Address,
        Envelope,
        MyResult,
        Reply,
    },
    std::collections::HashMap,
};

pub trait Processor {
    fn execute(&self, message: Envelope) -> MyResult<Reply>;
}

pub type ProcMap = HashMap<String, Box<dyn Processor>>;

pub trait Service {
    fn address(&self) -> &Address;
    fn process(&self, message: Envelope) -> MyResult<Reply>;
}
