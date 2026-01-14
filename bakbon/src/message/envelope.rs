use {
    super::{
        Headers,
        MyResult,
        route::Route,
    },
    crate::Endpoint,
    bytes::Bytes,
};

pub struct Envelope {
    headers: Headers,
    route:   Route,
    payload: Bytes,
}

impl Envelope {
    pub fn new(
        source: impl Into<String>,
        destination: impl Into<String>,
        payload: Bytes,
    ) -> MyResult<Self> {
        Ok(Self {
            headers: Headers::default(),
            route: Route::new(source, destination)?,
            payload,
        })
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers
            .get(key)
            .map(|v| v.as_str())
    }

    pub fn payload(&self) -> &Bytes { &self.payload }

    pub fn source(&self) -> &Endpoint { &self.route.source() }

    pub fn destination(&self) -> &Endpoint { &self.route.destination() }
}

//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_message_with_payload() {
        let source = "https://source.com";
        let destination = "https://destination.com";
        let payload = Bytes::from("random_payload");
        let msg_res = Envelope::new(source, destination, payload.clone());
        assert!(msg_res.is_ok());

        let msg = msg_res.unwrap();
        assert!(!msg.payload().is_empty());
        assert_eq!(msg.payload(), &payload);
    }

    #[test]
    fn build_message_with_empty_payload() {
        let source = "https://source.com";
        let destination = "https://destination.com";
        let payload: Bytes = Bytes::new();
        let msg_res = Envelope::new(source, destination, payload);
        assert!(msg_res.is_ok());

        let msg = msg_res.unwrap();
        assert!(msg.payload().is_empty());
    }

    #[test]
    fn build_message_with_metadata() -> MyResult<()> {
        let source = "https://source.com";
        let destination = "https://destination.com";
        let payload = Bytes::from("random_payload");

        let msg = Envelope::new(source, destination, payload)?
            .add_header("content-type", "text/plain")
            .add_header("encoding", "utf-8");

        let content_type = msg.get_header("content-type");
        assert!(content_type.is_some());
        assert_eq!(content_type.unwrap(), "text/plain");

        let encoding = msg.get_header("encoding");
        assert!(encoding.is_some());
        assert_eq!(encoding.unwrap(), "utf-8");

        Ok(())
    }
}
