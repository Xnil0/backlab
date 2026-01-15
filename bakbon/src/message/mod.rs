mod address;
mod envelope;
mod route;

use {
    crate::MyResult,
    std::collections::HashMap,
};
pub use {
    address::Address,
    envelope::Envelope,
};

pub type Reply = Option<Envelope>;
pub type Headers = HashMap<String, String>;
