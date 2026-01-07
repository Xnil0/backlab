//! Identity represents the ID configurations of a `Message`.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct MessageId(String);

impl MessageId {
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }

    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CorrelationId(String);

impl CorrelationId {
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }

    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CausationId(String);

impl CausationId {
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }

    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct TenantId(String);

impl TenantId {
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }

    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct TraceId(String);

impl TraceId {
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }

    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct SpanId(String);

impl SpanId {
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }

    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct IntegrityTag(String);

impl IntegrityTag {
    pub fn new(tag: impl Into<String>) -> Self { Self(tag.into()) }

    pub fn value(&self) -> &str { &self.0 }
}
