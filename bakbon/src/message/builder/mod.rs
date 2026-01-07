mod build;
mod message;

pub use message::Message;
use {
    super::{
        ContentType,
        DeliveryMode,
        Encoding,
        MessageKind,
        Method,
        Priority,
        Scope,
        delivery::OrderingKey,
        identity::{
            CausationId,
            CorrelationId,
            IntegrityTag,
            MessageId,
            SpanId,
            TenantId,
            TraceId,
        },
        semantics::Intent,
    },
    std::time::{
        Duration,
        SystemTime,
    },
};

pub struct Builder<T> {
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
    scope:        Scope,
    method:       Option<Method>,
    intent:       Option<Intent>,

    // Payload
    content_type: ContentType,
    encoding:     Encoding,
    body:         T,

    // Delivery
    delivery_mode: DeliveryMode,
    priority:      Priority,
    deadline:      Option<SystemTime>,
    delay:         Option<Duration>,
    retries:       u32,
    ttl:           Option<Duration>,
    ordering_key:  Option<OrderingKey>,

    // Security
    subject:       Option<String>,
    roles:         Vec<String>,
    permissions:   Vec<String>,
    integrity_tag: Option<IntegrityTag>,

    timestamp: SystemTime,
}

impl<T> Builder<T> {
    pub fn new(id: impl Into<String>, kind: MessageKind, body: T) -> Self {
        Self {
            id: MessageId::new(id.into()),
            correlation_id: None,
            causation_id: None,
            trace_id: None,
            span_id: None,
            tenant_id: None,
            kind,
            expect_reply: false,
            scope: Scope::default(),
            method: None,
            intent: None,
            content_type: ContentType::default(),
            encoding: Encoding::default(),
            body,
            delivery_mode: DeliveryMode::default(),
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
