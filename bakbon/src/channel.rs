use crate::{
    MyResult,
    message::Envelope,
};

pub trait Sender {
    fn send(&self, message: Envelope) -> MyResult<()>;
}

pub trait Receiver {
    fn receive(&self) -> MyResult<Envelope>;
}

pub struct Channel<'a, S, R>
where
    S: Sender,
    R: Receiver,
{
    from: &'a S,
    to:   &'a R,
}

impl<'a, S, R> Channel<'a, S, R>
where
    S: Sender,
    R: Receiver,
{
    pub fn new(from: &'a S, to: &'a R) -> Self { Self { from, to } }

    pub fn enqueue(&self, message: Envelope) -> MyResult<()> { self.from.send(message) }

    pub fn dequeue(&self) -> MyResult<Envelope> { self.to.receive() }

    pub fn from(&self) -> &S { &self.from }

    pub fn to(&self) -> &R { &self.to }
}
