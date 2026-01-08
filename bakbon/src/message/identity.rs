//! Identity represents the ID configurations of a `Message`.

use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct MessageId(String);

impl Default for MessageId {
    fn default() -> Self { Self(Uuid::new_v4().to_string()) }
}

impl From<String> for MessageId {
    fn from(value: String) -> Self { Self(value) }
}

impl AsRef<str> for MessageId {
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CorrelationId(String);

impl Default for CorrelationId {
    fn default() -> Self { Self(Uuid::new_v4().to_string()) }
}

impl From<String> for CorrelationId {
    fn from(value: String) -> Self { Self(value) }
}

impl AsRef<str> for CorrelationId {
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CausationId(String);

impl Default for CausationId {
    fn default() -> Self { Self(Uuid::new_v4().to_string()) }
}

impl From<String> for CausationId {
    fn from(value: String) -> Self { Self(value) }
}

impl AsRef<str> for CausationId {
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct TenantId(String);

impl Default for TenantId {
    fn default() -> Self { Self(Uuid::new_v4().to_string()) }
}

impl From<String> for TenantId {
    fn from(value: String) -> Self { Self(value) }
}

impl AsRef<str> for TenantId {
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct TraceId(String);

impl Default for TraceId {
    fn default() -> Self { Self(Uuid::new_v4().to_string()) }
}

impl From<String> for TraceId {
    fn from(value: String) -> Self { Self(value) }
}

impl AsRef<str> for TraceId {
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct SpanId(String);

impl Default for SpanId {
    fn default() -> Self { Self(Uuid::new_v4().to_string()) }
}

impl From<String> for SpanId {
    fn from(value: String) -> Self { Self(value) }
}

impl AsRef<str> for SpanId {
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct IntegrityTag(String);

impl IntegrityTag {
    pub fn new(tag: impl Into<String>) -> Self { Self(tag.into()) }

    pub fn value(&self) -> &str { &self.0 }
}
