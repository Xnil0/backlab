//! This structure is meant to represent the archetype of all communication
//! data units that exist regardless of the field.

use {
    super::Builder,
    crate::message::{
        ContentType,
        Encoding,
        Fanout,
        Guarantee,
        Intent,
        MessageKind,
        Method,
        OrderingKey,
        Priority,
        identity::{
            CausationId,
            CorrelationId,
            IntegrityTag,
            MessageId,
            SpanId,
            TenantId,
            TraceId,
        },
    },
    std::time::{
        Duration,
        SystemTime,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub struct Message<T> {
    // Identity
    pub(super) id:             MessageId,
    pub(super) correlation_id: Option<CorrelationId>,
    pub(super) causation_id:   Option<CausationId>,
    pub(super) trace_id:       Option<TraceId>,
    pub(super) span_id:        Option<SpanId>,
    pub(super) tenant_id:      Option<TenantId>,

    // Semantics
    pub(super) kind:         MessageKind,
    pub(super) expect_reply: bool,
    pub(super) method:       Option<Method>,
    pub(super) intent:       Option<Intent>,

    // Payload
    pub(super) content_type: ContentType,
    pub(super) encoding:     Encoding,
    pub(super) body:         T,

    // Delivery
    pub(super) guarantee:    Guarantee,
    pub(super) fanout:       Fanout,
    pub(super) priority:     Priority,
    pub(super) deadline:     Option<SystemTime>,
    pub(super) delay:        Option<Duration>,
    pub(super) retries:      u32,
    pub(super) ttl:          Option<Duration>,
    pub(super) ordering_key: Option<OrderingKey>,

    // Security
    pub(super) subject:       Option<String>,
    pub(super) roles:         Vec<String>,
    pub(super) permissions:   Vec<String>,
    pub(super) integrity_tag: Option<IntegrityTag>,

    pub(super) timestamp: SystemTime,
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
