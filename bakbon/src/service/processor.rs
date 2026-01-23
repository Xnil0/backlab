use {
    crate::{
        Envelope,
        Reply,
        Result,
    },
    std::collections::HashMap,
};

pub type ProcMap = HashMap<String, Box<dyn Processor>>;

pub trait Processor {
    fn execute(&self, message: Envelope) -> Result<Reply>;
}
