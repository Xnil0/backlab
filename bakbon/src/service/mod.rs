mod processor;

pub use processor::{
    ProcMap,
    Processor,
};
use {
    super::{
        Envelope,
        MyResult,
        Reply,
    },
    crate::Address,
};

pub trait Service {
    fn address(&self) -> &Address;
    fn process(&self, message: Envelope) -> MyResult<Reply>;
    fn duplicate(&self) -> Box<dyn Service>;
}
