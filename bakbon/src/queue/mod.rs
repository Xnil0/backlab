mod attributes;
mod builder;

use {
    crate::{
        Envelope,
        Error,
        Result,
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
    pub fn builder() -> QueueBuilder { QueueBuilder::default() }

    pub fn enqueue(&self, mut msg: Envelope) -> Result<()> {
        let mut buffer = self.buffer.lock()?;

        if let Some(capacity) = self.capacity {
            if buffer.len() >= capacity {
                return Err(Error::QueueFull(msg));
            };
        }
        if self.ttl.is_some() {
            let time_to_live = format!("{:?}", self.ttl.unwrap());
            msg.add_header("x-ttl", time_to_live.as_str())
        };
        match self.ordering {
            Ordering::Unordered => buffer.push_front(msg),
            _ => buffer.push_back(msg),
        }

        // NOTE -> Queue Providers (Kafka, RabbitMQ, etc),
        // Durability and Delivery Guarantee will be added.

        Ok(())
    }

    pub fn dequeue(&self) -> Result<Option<Envelope>> {
        let mut buffer = self.buffer.lock()?;

        match buffer.pop_front() {
            Some(message) => Ok(Some(message)),
            None => Ok(None),
        }
    }

    pub fn provider(&self) -> &str { self.provider.as_ref() }

    pub fn capacity(&self) -> Option<usize> { self.capacity }

    pub fn time_to_live(&self) -> Option<Duration> { self.ttl }

    pub fn ordering(&self) -> &str { self.ordering.as_ref() }

    pub fn durability(&self) -> &str { self.durability.as_ref() }

    pub fn delivery_guarantee(&self) -> &str {
        self.delivery_guarantee
            .as_ref()
    }

    pub fn len(&self) -> usize {
        let buffer = self.buffer.lock().unwrap();
        buffer.len()
    }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Address,
        bytes::Bytes,
    };

    const DST: &str = "tcp://queue.com";

    #[test]
    fn default_queue() {
        let queue = Queue::default();
        assert_eq!(queue.provider(), "memory")
    }

    #[test]
    fn build_queue() {
        let provider = "kafka";
        let capacity = 250;
        let ttl = Duration::from_secs(60);
        let ordering = "priority";
        let durability = "replicated";
        let guarantee = "exactly_once";

        let queue = Queue::builder()
            .provider(provider)
            .capacity(capacity)
            .time_to_live(ttl)
            .ordering(ordering)
            .durability(durability)
            .delivery_guarantee(guarantee)
            .build();

        assert_eq!(queue.provider(), provider);
        assert_eq!(queue.capacity(), Some(capacity));
        assert_eq!(queue.time_to_live(), Some(ttl));
        assert_eq!(queue.ordering(), ordering);
        assert_eq!(queue.durability(), durability);
        assert_eq!(queue.delivery_guarantee(), guarantee);
    }

    #[test]
    fn capacity_exceeded() -> Result<()> {
        let addr1 = Address::parse("tcp://first.com")?;
        let addr2 = Address::parse("tcp://second.com")?;
        let addr3 = Address::parse("tcp://third.com")?;
        let dst = Address::parse(DST)?;
        let payload = Bytes::from("Hello, Queue!");

        let msg1 = Envelope::new(addr1, dst.clone(), payload.clone());
        let msg2 = Envelope::new(addr2, dst.clone(), payload.clone());
        let msg3 = Envelope::new(addr3, dst, payload);

        let queue = Queue::builder()
            .capacity(2)
            .build();

        queue.enqueue(msg1)?;
        queue.enqueue(msg2)?;

        let result = queue.enqueue(msg3);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Error::QueueFull(_)
        ));

        Ok(())
    }

    #[test]
    fn empty_queue() {
        let queue = Queue::default();
        let msg = queue.dequeue();
        assert!(msg.is_ok());
        assert!(msg.unwrap().is_none())
    }

    #[test]
    fn enqueue_with_ttl() -> Result<()> {
        let ttl = Duration::from_secs(10);
        let src = Address::parse("http://service.com")?;
        let dst = Address::parse(DST)?;
        let payload = Bytes::default();

        let queue = Queue::builder()
            .time_to_live(ttl)
            .build();

        let msg = Envelope::new(src, dst, payload);
        queue.enqueue(msg)?;

        let msg = queue.dequeue()?;
        assert!(msg.is_some());

        let msg = msg.unwrap();
        let x_ttl = msg.get_header("x-ttl");
        assert!(x_ttl.is_some());
        assert_eq!(x_ttl.unwrap(), "10s");

        Ok(())
    }

    #[test]
    fn fifo_ordering() -> Result<()> {
        let addr1 = Address::parse("http://service1.com")?;
        let addr1_str = addr1.to_string();
        let dst = Address::parse(DST)?;
        let msg1 = Envelope::new(
            addr1,
            dst.clone(),
            Bytes::from("Hello, Queue!"),
        );

        let addr2 = Address::parse("http://service2.com")?;
        let addr2_str = addr2.to_string();
        let msg2 = Envelope::new(addr2, dst, Bytes::from("Hello, Queue!"));

        let queue = Queue::default();
        queue.enqueue(msg1)?;
        queue.enqueue(msg2)?;

        let msg = queue.dequeue()?;
        assert!(msg.is_some());
        let msg = msg.unwrap();
        assert_eq!(msg.source().to_string(), addr1_str);

        let msg = queue.dequeue()?;
        assert!(msg.is_some());
        let msg = msg.unwrap();
        assert_eq!(msg.source().to_string(), addr2_str);

        Ok(())
    }

    #[test]
    fn unordered_queue() -> Result<()> {
        let addr1 = Address::parse("http://service1.com")?;
        let addr1_str = addr1.to_string();
        let dst = Address::parse(DST)?;
        let msg1 = Envelope::new(
            addr1,
            dst.clone(),
            Bytes::from("Hello, Queue!"),
        );

        let addr2 = Address::parse("http://service2.com")?;
        let addr2_str = addr2.to_string();
        let msg2 = Envelope::new(addr2, dst, Bytes::from("Hello, Queue!"));

        let queue = Queue::builder()
            .ordering("unordered")
            .build();
        queue.enqueue(msg1)?;
        queue.enqueue(msg2)?;

        let msg = queue
            .dequeue()?
            .ok_or(Error::InvalidMessage)?;
        assert_eq!(msg.source().to_string(), addr2_str);

        let msg = queue
            .dequeue()?
            .ok_or(Error::InvalidMessage)?;
        assert_eq!(msg.source().to_string(), addr1_str);

        Ok(())
    }
}
