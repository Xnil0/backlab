#![allow(unused)]

use std::time::{
    Duration,
    SystemTime,
};

pub struct Envelope<T> {
    source:      Endpoint,
    destination: Endpoint,
    headers:     Vec<Header>,
    message:     Message<T>,
}

pub struct Endpoint {
    scheme:    Scheme,
    authority: String,
    path:      Option<String>,
}

pub struct Header(String, String);

pub enum Scheme {
    Tcp,
    Udp,
    Http,
    Https,
    Grpc,
    Serial,
    InProc,
    Custom(String),
}

/// This structure is meant to represent the archetype of all communication
/// models that exist regardless of the field.
pub struct Message<T> {
    identity:  Identity,
    semantics: Semantics,
    payload:   Payload<T>,
    delivery:  Option<Delivery>,
    tracing:   Option<Tracing>,
    tenancy:   Option<Tenancy>,
    security:  Option<Security>,
    timestamp: SystemTime,
}

pub struct Identity {
    message:     MessageId,
    correlation: Option<CorrelationId>,
    causation:   Option<CausationId>,
}

pub struct MessageId(String);
pub struct CorrelationId(String);
pub struct CausationId(String);

pub struct Semantics {
    category:     Category,
    scope:        Option<Scope>,
    method:       Option<Method>,
    intent:       Option<String>,
    expect_reply: bool,
}

pub enum Category {
    Command,
    Query,
    Reply,
    Event,
    Telemetry,
}

pub enum Scope {
    Unicast,
    Multicast,
    Broadcast,
}

pub enum Method {
    Create,
    Read,
    Update,
    Delete,
    Custom(String),
}

pub struct Payload<T> {
    content_type: ContentType,
    encoding:     Encoding,
    body:         T,
}

pub enum ContentType {
    Json,
    Text,
    Bincode,
    Protobuf,
    Avro,
    Custom(String),
}

pub enum Encoding {
    Utf8,
    Utf16,
    Binary,
    Gzip,
    Custom(String),
}

pub struct Delivery {
    mode:         Mode,
    ttl:          Option<Duration>,
    deadline:     Option<SystemTime>,
    priority:     Option<Priority>,
    ordering_key: Option<OrderingKey>,
    attempts:     Option<u32>,
    delay:        Option<Duration>,
}

pub struct OrderingKey(String);

pub enum Mode {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
}

pub enum Priority {
    Low,
    Normal,
    High,
}

pub struct Tracing {
    trace_id: Option<TraceId>,
    span_id:  Option<SpanId>,
}

pub struct TraceId(String);
pub struct SpanId(String);

pub struct Tenancy {
    tenant_id: Option<TenantId>,
}

pub struct TenantId(String);

pub struct Security {
    auth_context: Option<AuthContext>,
    signature:    Option<Signature>,
}

pub struct Signature(String);

pub struct AuthContext {
    subject: Option<String>,
    roles:   Vec<String>,
    scopes:  Vec<String>,
}
