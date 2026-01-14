use {
    bakbon::{
        Envelope,
        MyErr,
        MyResult,
        Receiver,
    },
    std::sync::mpsc,
};

pub struct MpscReceiver {
    address: String,
    inner:   mpsc::Receiver<Envelope>,
}

impl Receiver for MpscReceiver {
    fn receive(&self) -> MyResult<Envelope> {
        self.inner
            .recv()
            .map_err(|_| MyErr::ReceptionFailed)
    }
}

impl MpscReceiver {
    pub fn new(address: &str, inner: mpsc::Receiver<Envelope>) -> Self {
        Self {
            address: address.to_string(),
            inner,
        }
    }

    pub fn address(&self) -> &str { &self.address }
}
