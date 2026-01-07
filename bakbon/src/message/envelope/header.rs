pub struct Header(String, String);

impl Header {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Header(key.into(), value.into())
    }

    pub fn key(&self) -> &str { &self.0 }

    pub fn value(&self) -> &str { &self.1 }
}
