use {
    super::{
        Headers,
        route::Route,
    },
    crate::Address,
    bytes::Bytes,
};

/// Envelope is a message wrapper that contains headers, route, and
/// payload.
#[derive(Debug)]
pub struct Envelope {
    headers: Headers,
    route:   Route,
    payload: Bytes,
}

impl Envelope {
    pub fn new(src: Address, dst: Address, payload: Bytes) -> Self {
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

    pub fn into_reply(mut self, payload: Bytes) -> Self {
        self.route.swap_endpoints();
        self.payload = payload;
        self
    }

    pub fn payload(&self) -> &Bytes { &self.payload }

    pub fn source(&self) -> &Address { &self.route.source() }

    pub fn destination(&self) -> &Address { &self.route.destination() }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = "https://source.com";
    const DST: &str = "https://destination.com";

    #[test]
    fn new_message_with_payload() {
        let src = Address::parse(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let dst = Address::parse(DST);
        assert!(dst.is_ok());
        let dst = dst.unwrap();

        let payload = Bytes::from("random_payload");
        let msg = Envelope::new(src, dst, payload.clone());

        assert!(!msg.payload().is_empty());
        assert_eq!(msg.payload(), &payload);
    }

    #[test]
    fn new_message_with_empty_payload() {
        let src = Address::parse(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let dst = Address::parse(DST);
        assert!(dst.is_ok());
        let dst = dst.unwrap();

        let payload: Bytes = Bytes::new();
        let msg = Envelope::new(src, dst, payload);

        assert!(msg.payload().is_empty());
    }

    #[test]
    fn new_message_with_headers() {
        let src = Address::parse(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let dst = Address::parse(DST);
        assert!(dst.is_ok());
        let dst = dst.unwrap();

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
    fn copy_headers() {
        let src = Address::parse(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let dst = Address::parse(DST);
        assert!(dst.is_ok());
        let dst = dst.unwrap();

        let payload = Bytes::from("random_payload");

        let msg = Envelope::new(src.clone(), dst.clone(), payload.clone())
            .header("content-type", "text/plain")
            .header("encoding", "utf-8");

        let reply = msg.into_reply(payload);

        let content_type = reply.get_header("content-type");
        assert!(content_type.is_some());
        assert_eq!(content_type.unwrap(), "text/plain");

        let encoding = reply.get_header("encoding");
        assert!(encoding.is_some());
        assert_eq!(encoding.unwrap(), "utf-8");
    }
}
