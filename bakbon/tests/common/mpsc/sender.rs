use {
    bakbon::{
        Envelope,
        MyErr,
        MyResult,
        Sender,
    },
    std::sync::mpsc,
};

pub struct MpscSender {
    address: String,
    inner:   mpsc::Sender<Envelope>,
}

impl Sender for MpscSender {
    fn send(&self, message: Envelope) -> MyResult<()> {
        self.inner
            .send(message)
            .map_err(|_| MyErr::SendFailed)
    }
}

impl MpscSender {
    pub fn new(address: &str, inner: mpsc::Sender<Envelope>) -> Self {
        Self {
            address: address.to_string(),
            inner,
        }
    }

    pub fn address(&self) -> &str { &self.address }
}
