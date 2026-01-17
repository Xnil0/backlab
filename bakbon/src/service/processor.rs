use {
    crate::{
        Envelope,
        MyResult,
        Reply,
    },
    std::collections::HashMap,
};

pub type ProcMap = HashMap<String, Box<dyn Processor>>;

pub trait Processor {
    fn execute(&self, message: Envelope) -> MyResult<Reply>;
}
