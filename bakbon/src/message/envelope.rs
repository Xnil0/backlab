use std::collections::HashMap;

type Headers = HashMap<String, String>;

#[derive(Default)]
pub(super) struct Envelope {
    headers: Headers,
}

impl Envelope {
    pub(super) fn add_header(&mut self, key: &str, value: &str) {
        self.headers
            .insert(key.into(), value.into());
    }

    pub(super) fn is_empty(&self) -> bool { self.headers.is_empty() }

    pub(super) fn get_header(&self, key: &str) -> Option<&str> {
        self.headers
            .get(key)
            .map(|v| v.as_str())
    }
}
