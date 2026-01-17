mod attributes;
mod builder;

use {
    crate::{
        Envelope,
        MyErr,
        MyResult,
    },
    attributes::{
        DeliveryGuarantee,
        Durability,
        Ordering,
        QueueProvider,
    },
    builder::QueueBuilder,
    std::{
        collections::VecDeque,
        sync::Mutex,
        time::Duration,
    },
};

#[derive(Default)]
pub struct Queue {
    provider:           QueueProvider,
    buffer:             Mutex<VecDeque<Envelope>>,
    capacity:           Option<usize>,
    ttl:                Option<Duration>,
    ordering:           Ordering,
    durability:         Durability,
    delivery_guarantee: DeliveryGuarantee,
}

impl Queue {
    pub fn builder() -> QueueBuilder { QueueBuilder::new() }

    pub fn enqueue(&self, mut message: Envelope) -> MyResult<()> {
        let mut buffer = self.buffer.lock()?;

        if let Some(capacity) = self.capacity {
            if buffer.len() >= capacity {
                return Err(MyErr::QueueFull);
            };
        }
        if self.ttl.is_some() {
            let time_to_live = format!("{:?}", self.ttl);
            message.add_header("x-ttl", time_to_live.as_str())
        };
        match self.ordering {
            Ordering::Unordered => buffer.push_front(message),
            _ => buffer.push_back(message),
        }

        // NOTE -> Queue Providers (Kafka, RabbitMQ, etc),
        // Durability and Delivery Guarantee will be added.

        Ok(())
    }

    pub fn dequeue(&self) -> MyResult<Option<Envelope>> {
        let mut buffer = self.buffer.lock()?;

        match buffer.pop_front() {
            Some(message) => Ok(Some(message)),
            None => Ok(None),
        }
    }
}
