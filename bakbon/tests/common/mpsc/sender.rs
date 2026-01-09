use {
    bakbon::{
        Envelope,
        MyErr,
        MyResult,
        Sender,
    },
    std::sync::mpsc,
};

pub struct MpscSender<P> {
    address: String,
    inner:   mpsc::Sender<Envelope<P>>,
}

impl<P> MpscSender<P> {
    pub fn new(address: &str, inner: mpsc::Sender<Envelope<P>>) -> Self {
        Self {
            address: address.to_string(),
            inner,
        }
    }

    pub fn address(&self) -> &str { &self.address }
}

impl<P> Sender<P> for MpscSender<P> {
    fn send(&self, message: Envelope<P>) -> MyResult<()> {
        self.inner
            .send(message)
            .map_err(|_| MyErr::SendFailed)
    }
}
