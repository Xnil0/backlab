use {
    super::{
        Builder,
        Message,
    },
    crate::message::{
        ContentType,
        Encoding,
        Fanout,
        Guarantee,
        Intent,
        Method,
        OrderingKey,
        Priority,
        identity::IntegrityTag,
    },
    std::time::{
        Duration,
        SystemTime,
    },
};

impl<T> Builder<T> {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        let id_str = id.into();
        if !id_str.is_empty() {
            self.id = Some(id_str.into());
        }
        self
    }

    pub fn correlation_id(mut self, id: impl Into<String>) -> Self {
        let id_str = id.into();
        if !id_str.is_empty() {
            self.correlation_id = Some(id_str.into());
        }
        self
    }

    pub fn causation_id(mut self, id: impl Into<String>) -> Self {
        let id_str = id.into();
        if !id_str.is_empty() {
            self.causation_id = Some(id_str.into());
        }
        self
    }

    pub fn trace_id(mut self, id: impl Into<String>) -> Self {
        let id_str = id.into();
        if !id_str.is_empty() {
            self.trace_id = Some(id_str.into());
        }
        self
    }

    pub fn span_id(mut self, id: impl Into<String>) -> Self {
        let id_str = id.into();
        if !id_str.is_empty() {
            self.span_id = Some(id_str.into());
        }
        self
    }

    pub fn tenant_id(mut self, id: impl Into<String>) -> Self {
        let id_str = id.into();
        if !id_str.is_empty() {
            self.tenant_id = Some(id_str.into());
        }
        self
    }

    pub fn expect_reply(mut self) -> Self {
        self.expect_reply = true;
        self
    }

    pub fn fanout(mut self, fanout: Fanout) -> Self {
        self.fanout = fanout;
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

    pub fn guarantee(mut self, mode: Guarantee) -> Self {
        self.guarantee = mode;
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
            id:             self.id.unwrap_or_default(),
            correlation_id: self.correlation_id,
            causation_id:   self.causation_id,
            trace_id:       self.trace_id,
            span_id:        self.span_id,
            tenant_id:      self.tenant_id,

            kind:         self.kind,
            expect_reply: self.expect_reply,
            fanout:       self.fanout,
            method:       self.method,
            intent:       self.intent,

            content_type: self.content_type,
            encoding:     self.encoding,
            body:         self.body,

            guarantee:    self.guarantee,
            priority:     self.priority,
            deadline:     self.deadline,
            delay:        self.delay,
            retries:      self.retries,
            ttl:          self.ttl,
            ordering_key: self.ordering_key,

            subject:       self.subject,
            roles:         self.roles,
            permissions:   self.permissions,
            integrity_tag: self.integrity_tag,

            timestamp: self.timestamp,
        }
    }
}
