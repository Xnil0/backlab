mod build;
mod message;

pub use message::Message;
use {
    super::{
        ContentType,
        Encoding,
        Fanout,
        Guarantee,
        MessageKind,
        Method,
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
    crate::message::{
        Intent,
        OrderingKey,
    },
    std::time::{
        Duration,
        SystemTime,
    },
};

pub struct Builder<T> {
    // Identity
    id:             Option<MessageId>,
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

impl<T> Builder<T> {
    pub fn new(kind: MessageKind, body: T) -> Self {
        Self {
            id: None,
            correlation_id: None,
            causation_id: None,
            trace_id: None,
            span_id: None,
            tenant_id: None,
            kind,
            expect_reply: false,
            fanout: Fanout::default(),
            method: None,
            intent: None,
            content_type: ContentType::default(),
            encoding: Encoding::default(),
            body,
            guarantee: Guarantee::default(),
            priority: Priority::default(),
            deadline: None,
            delay: None,
            retries: 0,
            ttl: None,
            ordering_key: None,
            subject: None,
            roles: Vec::new(),
            permissions: Vec::new(),
            integrity_tag: None,
            timestamp: SystemTime::now(),
        }
    }
}
