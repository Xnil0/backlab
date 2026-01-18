use {
    super::{
        Headers,
        route::Route,
    },
    crate::Address,
    bytes::Bytes,
};

#[derive(Debug)]
pub struct Envelope {
    headers: Headers,
    route:   Route,
    payload: Bytes,
}

impl Envelope {
    pub fn new(src: Address, dst: impl ToString, payload: Bytes) -> Self {
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

    pub fn copy_headers(&mut self, msg: Self) {
        for (k, v) in msg.headers {
            if self.headers.get(&k).is_none() {
                self.headers.insert(k, v);
            }
        }
    }

    pub fn payload(&self) -> &Bytes { &self.payload }

    pub fn source(&self) -> &Address { &self.route.source() }

    pub fn destination(&self) -> &str { &self.route.destination() }
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
        let src = Address::new(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let payload = Bytes::from("random_payload");
        let msg = Envelope::new(src, DST, payload.clone());

        assert!(!msg.payload().is_empty());
        assert_eq!(msg.payload(), &payload);
    }

    #[test]
    fn new_message_with_empty_payload() {
        let src = Address::new(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let payload: Bytes = Bytes::new();
        let msg = Envelope::new(src, DST, payload);

        assert!(msg.payload().is_empty());
    }

    #[test]
    fn new_message_with_headers() {
        let src = Address::new(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let payload = Bytes::from("random_payload");

        let msg = Envelope::new(src, DST, payload)
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
        let src = Address::new(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let payload = Bytes::from("random_payload");

        let msg = Envelope::new(src, DST, payload.clone())
            .header("content-type", "text/plain")
            .header("encoding", "utf-8");

        let dst = Address::new(DST);
        assert!(dst.is_ok());
        let dst = dst.unwrap();

        let mut reply = Envelope::new(dst, SRC, payload.clone());
        reply.copy_headers(msg);

        let content_type = reply.get_header("content-type");
        assert!(content_type.is_some());
        assert_eq!(content_type.unwrap(), "text/plain");

        let encoding = reply.get_header("encoding");
        assert!(encoding.is_some());
        assert_eq!(encoding.unwrap(), "utf-8");
    }
}
