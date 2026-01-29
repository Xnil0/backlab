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

pub trait Processor: Debug {
    fn execute(&self, message: Envelope) -> Result<Reply>;
}

pub type ProcMap = HashMap<String, Box<dyn Processor>>;
