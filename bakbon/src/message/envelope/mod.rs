pub mod endpoint;
pub mod header;
pub mod scheme;

use {
    super::Message,
    endpoint::Endpoint,
    header::Header,
};

pub struct Envelope<T> {
    source:      Endpoint,
    destination: Endpoint,
    headers:     Vec<Header>,
    message:     Message<T>,
}

impl<T> Envelope<T> {
    pub fn new(source: Endpoint, destination: Endpoint, message: Message<T>) -> Self {
        Self {
            source,
            destination,
            headers: vec![],
            message,
        }
    }

    pub fn add_header(mut self, header: Header) -> Self {
        self.headers.push(header);
        self
    }

    pub fn source(&self) -> &Endpoint { &self.source }

    pub fn destination(&self) -> &Endpoint { &self.destination }

    pub fn headers(&self) -> &Vec<Header> { &self.headers }

    pub fn message(&self) -> &Message<T> { &self.message }
}
