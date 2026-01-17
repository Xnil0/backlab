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

#[derive(Default)]
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
    pub fn provider(mut self, provider: &str) -> Self {
        self.provider = QueueProvider::from(provider);
        self
    }

    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self
    }

    pub fn time_to_live(mut self, ttl: Duration) -> Self {
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

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_queue_with_provider() {
        let provider_str = "redis";
        let queue = QueueBuilder::default()
            .provider(provider_str)
            .build();
        assert_eq!(queue.provider(), provider_str);
    }

    #[test]
    fn build_queue_with_capacity() {
        let capacity = 100;
        let queue = QueueBuilder::default()
            .capacity(capacity)
            .build();
        assert_eq!(queue.capacity(), Some(capacity));
    }

    #[test]
    fn build_queue_with_ttl() {
        let ttl = Duration::from_secs(60);
        let queue = QueueBuilder::default()
            .time_to_live(ttl)
            .build();
        assert_eq!(queue.time_to_live(), Some(ttl));
    }

    #[test]
    fn build_queue_with_ordering() {
        let ordering_str = "fifo";
        let queue = QueueBuilder::default()
            .ordering(ordering_str)
            .build();
        assert_eq!(queue.ordering(), ordering_str);
    }

    #[test]
    fn build_queue_with_durability() {
        let durability_str = "replicated";
        let queue = QueueBuilder::default()
            .durability(durability_str)
            .build();
        assert_eq!(queue.durability(), durability_str);
    }

    #[test]
    fn build_queue_with_delivery_guarantee() {
        let guarantee_str = "at_least_once";
        let queue = QueueBuilder::default()
            .delivery_guarantee(guarantee_str)
            .build();
        assert_eq!(queue.delivery_guarantee(), guarantee_str);
    }
}
