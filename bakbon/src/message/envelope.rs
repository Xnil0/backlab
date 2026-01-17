use {
    super::{
        Headers,
        route::Route,
    },
    bytes::Bytes,
};

pub struct Envelope {
    headers: Headers,
    route:   Route,
    payload: Bytes,
}

impl Envelope {
    pub fn new(src: impl ToString, dst: impl ToString, payload: Bytes) -> Self {
        Self {
            headers: Headers::default(),
            route: Route::new(src, dst),
            payload,
        }
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn add_header(&mut self, k: &str, v: &str) {
        self.headers
            .insert(k.to_string(), v.to_string());
    }

    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers
            .get(key)
            .map(|v| v.as_str())
    }

    pub fn payload(&self) -> &Bytes { &self.payload }

    pub fn source(&self) -> &str { &self.route.source() }

    pub fn destination(&self) -> &str { &self.route.destination() }

    pub fn reply(mut self, payload: Bytes) -> Self {
        self.route = Route::new(
            self.destination().to_string(),
            self.source().to_string(),
        );
        self.payload = payload;
        self
    }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_message_with_payload() {
        let src = "https://source.com";
        let dst = "https://destination.com";
        let payload = Bytes::from("random_payload");
        let msg = Envelope::new(src, dst, payload.clone());

        assert!(!msg.payload().is_empty());
        assert_eq!(msg.payload(), &payload);
    }

    #[test]
    fn new_message_with_empty_payload() {
        let src = "https://source.com";
        let dst = "https://destination.com";
        let payload: Bytes = Bytes::new();
        let msg = Envelope::new(src, dst, payload);

        assert!(msg.payload().is_empty());
    }

    #[test]
    fn new_message_with_headers() {
        let src = "https://source.com";
        let dst = "https://destination.com";
        let payload = Bytes::from("random_payload");

        let msg = Envelope::new(src, dst, payload)
            .header("content-type", "text/plain")
            .header("encoding", "utf-8");

        let content_type = msg.get_header("content-type");
        assert!(content_type.is_some());
        assert_eq!(content_type.unwrap(), "text/plain");

        let encoding = msg.get_header("encoding");
        assert!(encoding.is_some());
        assert_eq!(encoding.unwrap(), "utf-8");
    }

    #[test]
    fn envelope_reply() {
        let src = "https://source.com";
        let dst = "https://destination.com";
        let payload = Bytes::from("How old are you?");

        let msg = Envelope::new(src, dst, payload)
            .header("content-type", "text/plain")
            .header("encoding", "utf-8");

        let new_payload = Bytes::from("{age: 31}");
        let reply = msg
            .reply(new_payload.clone())
            .header("content-type", "application/json");

        let content_type = reply.get_header("content-type");
        assert!(content_type.is_some());
        assert_eq!(content_type.unwrap(), "application/json");

        let encoding = reply.get_header("encoding");
        assert!(encoding.is_some());
        assert_eq!(encoding.unwrap(), "utf-8");

        assert_eq!(reply.payload(), &new_payload);
    }
}
