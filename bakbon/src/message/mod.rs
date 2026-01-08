// #![allow(unused)]

mod builder;
mod content_type;
mod encoding;
mod endpoint;
mod envelope;
mod header;
mod identity;
mod kind;
mod method;
mod mode;
mod priority;
mod scheme;
mod scope;

pub use {
    builder::Message,
    content_type::ContentType,
    encoding::Encoding,
    kind::MessageKind,
    method::Method,
    mode::Guarantee,
    priority::Priority,
    scope::Fanout,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Intent(String);

impl Intent {
    pub fn new(intent: impl Into<String>) -> Self { Self(intent.into()) }

    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct OrderingKey(String);

impl OrderingKey {
    pub fn new(key: impl Into<String>) -> Self { Self(key.into()) }

    pub fn value(&self) -> &str { &self.0 }
}
