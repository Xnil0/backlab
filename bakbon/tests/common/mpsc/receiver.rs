use {
    bakbon::{
        Envelope,
        MyErr,
        MyResult,
        Receiver,
    },
    std::sync::mpsc,
};

pub struct MpscReceiver<P> {
    address: String,
    inner:   mpsc::Receiver<Envelope<P>>,
}

impl<P> MpscReceiver<P> {
    pub fn new(address: &str, inner: mpsc::Receiver<Envelope<P>>) -> Self {
        Self {
            address: address.to_string(),
            inner,
        }
    }

    pub fn address(&self) -> &str { &self.address }
}

impl<P> Receiver<P> for MpscReceiver<P> {
    fn receive(&self) -> MyResult<Envelope<P>> {
        self.inner
            .recv()
            .map_err(|_| MyErr::ReceptionFailed)
    }
}
