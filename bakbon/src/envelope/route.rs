#[derive(Default)]
pub(super) struct EndpointAddr(String);

impl<T> From<T> for EndpointAddr
where
    T: ToString,
{
    fn from(value: T) -> Self { Self(value.to_string()) }
}

pub(super) struct Route {
    source:      EndpointAddr,
    destination: EndpointAddr,
}

impl Route {
    pub fn new(source: impl Into<EndpointAddr>, destination: impl Into<EndpointAddr>) -> Self {
        Self {
            source:      source.into(),
            destination: destination.into(),
        }
    }

    pub fn source(&self) -> &str { &self.source.0 }

    pub fn destination(&self) -> &str { &self.destination.0 }
}
