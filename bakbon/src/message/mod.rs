mod envelope;
mod route;

use std::collections::HashMap;

pub use envelope::Envelope;

pub type Reply = Option<Envelope>;
pub type Headers = HashMap<String, String>;
