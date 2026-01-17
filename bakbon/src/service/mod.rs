mod processor;

pub use processor::{
    ProcMap,
    Processor,
};

use super::{
    Envelope,
    MyResult,
    Reply,
};

pub trait Service {
    fn address(&self) -> &str;
    fn process(&self, message: Envelope) -> MyResult<Reply>;
    fn duplicate(&self) -> Box<dyn Service>;
}
