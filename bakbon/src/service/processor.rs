use {
    crate::{
        Envelope,
        Reply,
        Result,
    },
    std::{
        collections::HashMap,
        fmt::Debug,
    },
};

pub type ProcMap = HashMap<String, Box<dyn Processor>>;

pub trait Processor: Debug {
    fn execute(&self, message: Envelope) -> Result<Reply>;
}
