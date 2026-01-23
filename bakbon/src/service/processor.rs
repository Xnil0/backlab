use {
    crate::{
        Envelope,
        Result,
        Reply,
    },
    std::collections::HashMap,
};

pub type ProcMap = HashMap<String, Box<dyn Processor>>;

pub trait Processor {
    fn execute(&self, message: Envelope) -> Result<Reply>;
}
