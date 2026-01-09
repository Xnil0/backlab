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
