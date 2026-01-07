use {
    super::{
        Builder,
        Message,
    },
    crate::message::{
        ContentType,
        DeliveryMode,
        Encoding,
        Method,
        Priority,
        Scope,
        delivery::OrderingKey,
        identity::{
            CausationId,
            CorrelationId,
            IntegrityTag,
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

impl<T> Builder<T> {
    pub fn correlation(mut self, id: impl Into<String>) -> Self {
        self.correlation_id = Some(CorrelationId::new(id.into()));
        self
    }

    pub fn causation(mut self, id: impl Into<String>) -> Self {
        self.causation_id = Some(CausationId::new(id.into()));
        self
    }

    pub fn trace(mut self, id: impl Into<String>) -> Self {
        self.trace_id = Some(TraceId::new(id.into()));
        self
    }

    pub fn span(mut self, id: impl Into<String>) -> Self {
        self.span_id = Some(SpanId::new(id.into()));
        self
    }

    pub fn tenant(mut self, id: impl Into<String>) -> Self {
        self.tenant_id = Some(TenantId::new(id.into()));
        self
    }

    pub fn expect_reply(mut self) -> Self {
        self.expect_reply = true;
        self
    }

    pub fn scope(mut self, scope: Scope) -> Self {
        self.scope = scope;
        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn intent(mut self, intent: impl Into<String>) -> Self {
        self.intent = Some(Intent::new(intent.into()));
        self
    }

    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = content_type;
        self
    }

    pub fn encoding(mut self, encoding: Encoding) -> Self {
        self.encoding = encoding;
        self
    }

    pub fn delivery_mode(mut self, mode: DeliveryMode) -> Self {
        self.delivery_mode = mode;
        self
    }

    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn deadline(mut self, deadline: SystemTime) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    pub fn ttl(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn ordering_key(mut self, ordering_key: impl Into<String>) -> Self {
        self.ordering_key = Some(OrderingKey::new(ordering_key.into()));
        self
    }

    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn add_role(mut self, role: impl Into<String>) -> Self {
        self.roles.push(role.into());
        self
    }

    pub fn add_permission(mut self, permission: impl Into<String>) -> Self {
        self.permissions
            .push(permission.into());
        self
    }

    pub fn integrity_tag(mut self, integrity_tag: impl Into<String>) -> Self {
        self.integrity_tag = Some(IntegrityTag::new(integrity_tag.into()));
        self
    }

    pub fn build(self) -> Message<T> {
        Message {
            id:             self.id,
            correlation_id: self.correlation_id,
            causation_id:   self.causation_id,
            trace_id:       self.trace_id,
            span_id:        self.span_id,
            tenant_id:      self.tenant_id,

            kind:         self.kind,
            expect_reply: self.expect_reply,
            scope:        self.scope,
            method:       self.method,
            intent:       self.intent,

            content_type: self.content_type,
            encoding:     self.encoding,
            body:         self.body,

            delivery_mode: self.delivery_mode,
            priority:      self.priority,
            deadline:      self.deadline,
            delay:         self.delay,
            retries:       self.retries,
            ttl:           self.ttl,
            ordering_key:  self.ordering_key,

            subject:       self.subject,
            roles:         self.roles,
            permissions:   self.permissions,
            integrity_tag: self.integrity_tag,

            timestamp: self.timestamp,
        }
    }
}
