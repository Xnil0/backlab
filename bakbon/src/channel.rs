use {
    crate::{
        MyResult,
        envelope::Envelope,
    },
    std::marker::PhantomData,
};

pub trait Sender<P> {
    fn send(&self, message: Envelope<P>) -> MyResult<()>;
}

pub trait Receiver<P> {
    fn receive(&self) -> MyResult<Envelope<P>>;
}

pub trait Endpoint<P>: Sender<P> + Receiver<P> {
    fn ping(&self, message: Envelope<P>) -> MyResult<Envelope<P>> {
        self.send(message)?;
        self.receive()
    }
}

pub struct Channel<S, R, P>
where
    S: Sender<P>,
    R: Receiver<P>,
{
    sender:   S,
    receiver: R,
    _phantom: PhantomData<P>,
}

impl<S, R, P> Channel<S, R, P>
where
    S: Sender<P>,
    R: Receiver<P>,
{
    pub fn new(sender: S, receiver: R) -> Self {
        Self {
            sender,
            receiver,
            _phantom: PhantomData,
        }
    }

    pub fn enqueue(&self, message: Envelope<P>) -> MyResult<()> { self.sender.send(message) }

    pub fn dequeue(&self) -> MyResult<Envelope<P>> { self.receiver.receive() }

    pub fn sender(&self) -> &S { &self.sender }

    pub fn receiver(&self) -> &R { &self.receiver }
}
