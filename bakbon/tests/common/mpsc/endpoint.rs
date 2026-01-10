use {
    bakbon::{
        Endpoint,
        Envelope,
        MyErr,
        MyResult,
        Receiver,
        Sender,
    },
    std::sync::mpsc,
};

pub struct MpscEndpoint<P> {
    address:  String,
    sender:   mpsc::Sender<Envelope<P>>,
    receiver: mpsc::Receiver<Envelope<P>>,
}

impl<P> Endpoint<P> for MpscEndpoint<P> {}

impl<P> Sender<P> for MpscEndpoint<P> {
    fn send(&self, message: Envelope<P>) -> MyResult<()> {
        self.sender
            .send(message)
            .map_err(|_| MyErr::SendFailed)
    }
}

impl<P> Receiver<P> for MpscEndpoint<P> {
    fn receive(&self) -> MyResult<Envelope<P>> {
        self.receiver
            .recv()
            .map_err(|_| MyErr::ReceptionFailed)
    }
}

impl<P> MpscEndpoint<P> {
    pub fn new(
        address: &str,
        sender: mpsc::Sender<Envelope<P>>,
        receiver: mpsc::Receiver<Envelope<P>>,
    ) -> Self {
        Self {
            address: address.to_string(),
            sender,
            receiver,
        }
    }

    pub fn address(&self) -> &str { &self.address }
}
