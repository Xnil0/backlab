use std::collections::HashMap;

type Headers = HashMap<String, String>;

#[derive(Default)]
pub struct Message<P> {
    headers: Headers,
    payload: P,
}

impl<P> Message<P> {
    pub fn new(data: P) -> Self {
        Self {
            headers: Headers::default(),
            payload: data.into(),
        }
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn payload(&self) -> &P { &self.payload }

    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers
            .get(key)
            .map(|v| v.as_str())
    }
}

//

#[cfg(test)]
mod tests {
    use super::Message;

    #[test]
    fn build_default_message() {
        let msg: Message<Vec<u8>> = Message::default();
        assert!(msg.payload().is_empty());
    }

    #[test]
    fn build_message_with_payload() {
        let payload = "random_payload";
        let msg = Message::new(payload);

        assert!(!msg.payload().is_empty());
        assert_eq!(msg.payload(), &payload);
    }

    #[test]
    fn build_message_with_empty_payload() {
        let payload: Vec<u8> = vec![];
        let msg = Message::new(payload);

        assert!(msg.payload().is_empty());
    }

    #[test]
    fn build_message_with_metadata() {
        let payload = String::from("random_payload");

        let msg = Message::new(payload)
            .add_header("content-type", "text/plain")
            .add_header("encoding", "utf-8");

        let content_type = msg.get_header("content-type");
        assert!(content_type.is_some());
        assert_eq!(content_type.unwrap(), "text/plain");

        let encoding = msg.get_header("encoding");
        assert!(encoding.is_some());
        assert_eq!(encoding.unwrap(), "utf-8");
    }
}
