mod envelope;

use envelope::Envelope;

type Payload = Vec<u8>;

#[derive(Default)]
pub struct Message {
    envelope: Envelope,
    payload:  Payload,
}

impl Message {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self {
            envelope: Envelope::default(),
            payload:  data.into(),
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.envelope
            .add_header(key, value);
        self
    }

    pub fn payload(&self) -> &Payload { &self.payload }

    pub fn has_meta(&self) -> bool { !self.envelope.is_empty() }

    pub fn meta(&self, key: &str) -> Option<&str> { self.envelope.get_header(key) }
}
