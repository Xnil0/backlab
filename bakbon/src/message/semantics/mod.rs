mod kind;
mod method;
mod scope;

pub use {
    kind::MessageKind,
    method::Method,
    scope::Scope,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Intent(String);

impl Intent {
    pub fn new(intent: impl Into<String>) -> Self { Self(intent.into()) }

    pub fn value(&self) -> &str { &self.0 }
}
