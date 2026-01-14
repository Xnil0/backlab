use {
    bakbon::{
        Envelope,
        MyErr,
        MyResult,
        Receiver,
        Sender,
    },
    std::sync::mpsc,
};

pub struct MpscEndpoint {
    address:  String,
    sender:   mpsc::Sender<Envelope>,
    receiver: mpsc::Receiver<Envelope>,
}

impl Sender for MpscEndpoint {
    fn send(&self, message: Envelope) -> MyResult<()> {
        self.sender
            .send(message)
            .map_err(|_| MyErr::SendFailed)
    }
}

impl Receiver for MpscEndpoint {
    fn receive(&self) -> MyResult<Envelope> {
        self.receiver
            .recv()
            .map_err(|_| MyErr::ReceptionFailed)
    }
}

impl MpscEndpoint {
    pub fn new(
        address: &str,
        sender: mpsc::Sender<Envelope>,
        receiver: mpsc::Receiver<Envelope>,
    ) -> Self {
        Self {
            address: address.to_string(),
            sender,
            receiver,
        }
    }

    pub fn address(&self) -> &str { &self.address }
}
