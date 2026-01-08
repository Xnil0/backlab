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

use {
    builder::Builder,
    identity::{
        CausationId,
        CorrelationId,
        IntegrityTag,
        MessageId,
        SpanId,
        TenantId,
        TraceId,
    },
    std::time::{
        Duration,
        SystemTime,
    },
};
pub use {
    content_type::ContentType,
    encoding::Encoding,
    kind::MessageKind,
    method::Method,
    mode::Guarantee,
    priority::Priority,
    scope::Fanout,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Message<T> {
    // Identity
    id:             MessageId,
    correlation_id: Option<CorrelationId>,
    causation_id:   Option<CausationId>,
    trace_id:       Option<TraceId>,
    span_id:        Option<SpanId>,
    tenant_id:      Option<TenantId>,

    // Semantics
    kind:         MessageKind,
    expect_reply: bool,
    method:       Option<Method>,
    intent:       Option<Intent>,

    // Payload
    content_type: ContentType,
    encoding:     Encoding,
    body:         T,

    // Delivery
    guarantee:    Guarantee,
    fanout:       Fanout,
    priority:     Priority,
    deadline:     Option<SystemTime>,
    delay:        Option<Duration>,
    retries:      u32,
    ttl:          Option<Duration>,
    ordering_key: Option<OrderingKey>,

    // Security
    subject:       Option<String>,
    roles:         Vec<String>,
    permissions:   Vec<String>,
    integrity_tag: Option<IntegrityTag>,

    timestamp: SystemTime,
}

impl<T> Message<T> {
    pub fn builder(kind: MessageKind, body: T) -> Builder<T> { Builder::new(kind, body) }

    pub fn id(&self) -> &str { self.id.as_ref() }

    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id
            .as_ref()
            .map(|id| id.as_ref())
    }

    pub fn causation_id(&self) -> Option<&str> {
        self.causation_id
            .as_ref()
            .map(|id| id.as_ref())
    }

    pub fn trace_id(&self) -> Option<&str> {
        self.trace_id
            .as_ref()
            .map(|id| id.as_ref())
    }

    pub fn span_id(&self) -> Option<&str> {
        self.span_id
            .as_ref()
            .map(|id| id.as_ref())
    }

    pub fn tenant_id(&self) -> Option<&str> {
        self.tenant_id
            .as_ref()
            .map(|id| id.as_ref())
    }

    pub fn kind(&self) -> &MessageKind { &self.kind }

    pub fn expect_reply(&self) -> bool { self.expect_reply }

    pub fn fanout(&self) -> &Fanout { &self.fanout }

    pub fn method(&self) -> Option<&Method> { self.method.as_ref() }

    pub fn intent(&self) -> Option<&str> {
        self.intent
            .as_ref()
            .map(|i| i.value())
    }

    pub fn content_type(&self) -> &ContentType { &self.content_type }

    pub fn encoding(&self) -> &Encoding { &self.encoding }

    pub fn body(&self) -> &T { &self.body }

    pub fn guarantee(&self) -> &Guarantee { &self.guarantee }

    pub fn priority(&self) -> &Priority { &self.priority }

    pub fn deadline(&self) -> Option<&SystemTime> { self.deadline.as_ref() }

    pub fn delay(&self) -> Option<&Duration> { self.delay.as_ref() }

    pub fn retries(&self) -> u32 { self.retries }

    pub fn time_to_live(&self) -> Option<&Duration> { self.ttl.as_ref() }

    pub fn ordering_key(&self) -> Option<&str> {
        self.ordering_key
            .as_ref()
            .map(|o| o.value())
    }

    pub fn subject(&self) -> Option<&str> {
        self.subject
            .as_ref()
            .map(|s| s.as_str())
    }

    pub fn roles(&self) -> &[String] { &self.roles }

    pub fn permissions(&self) -> &[String] { &self.permissions }

    pub fn integrity_tag(&self) -> Option<&str> {
        self.integrity_tag
            .as_ref()
            .map(|tag| tag.value())
    }

    pub fn timestamp(&self) -> &SystemTime { &self.timestamp }
}

#[derive(Debug, PartialEq, Eq)]
struct Intent(String);

impl Intent {
    pub fn new(intent: impl Into<String>) -> Self { Self(intent.into()) }

    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, PartialEq, Eq)]
struct OrderingKey(String);

impl OrderingKey {
    pub fn new(key: impl Into<String>) -> Self { Self(key.into()) }

    pub fn value(&self) -> &str { &self.0 }
}
