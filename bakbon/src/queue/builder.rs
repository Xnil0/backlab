use {
    super::{
        DeliveryGuarantee,
        Durability,
        Envelope,
        Ordering,
        Queue,
        QueueProvider,
    },
    std::{
        collections::VecDeque,
        sync::Mutex,
        time::Duration,
    },
};

pub struct QueueBuilder {
    provider:           QueueProvider,
    buffer:             Mutex<VecDeque<Envelope>>,
    capacity:           Option<usize>,
    ttl:                Option<Duration>,
    ordering:           Ordering,
    durability:         Durability,
    delivery_guarantee: DeliveryGuarantee,
}

impl QueueBuilder {
    pub fn new() -> Self {
        Self {
            provider:           QueueProvider::default(),
            buffer:             Mutex::new(VecDeque::new()),
            capacity:           None,
            ttl:                None,
            ordering:           Ordering::default(),
            durability:         Durability::default(),
            delivery_guarantee: DeliveryGuarantee::default(),
        }
    }

    pub fn provider(mut self, provider: &str) -> Self {
        self.provider = QueueProvider::from(provider);
        self
    }

    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self
    }

    pub fn ttl(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn ordering(mut self, ordering: &str) -> Self {
        self.ordering = Ordering::from(ordering);
        self
    }

    pub fn durability(mut self, durability: &str) -> Self {
        self.durability = Durability::from(durability);
        self
    }

    pub fn delivery_guarantee(mut self, delivery_guarantee: &str) -> Self {
        self.delivery_guarantee = DeliveryGuarantee::from(delivery_guarantee);
        self
    }

    pub fn build(self) -> Queue {
        Queue {
            provider:           self.provider,
            buffer:             self.buffer,
            capacity:           self.capacity,
            ttl:                self.ttl,
            ordering:           self.ordering,
            durability:         self.durability,
            delivery_guarantee: self.delivery_guarantee,
        }
    }
}
