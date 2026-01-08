use super::scheme::Scheme;

pub struct Endpoint {
    scheme:    Scheme,
    authority: String,
    path:      Option<String>,
}

impl Endpoint {
    pub fn new(scheme: Scheme, authority: impl Into<String>) -> Self {
        Endpoint {
            scheme,
            authority: authority.into(),
            path: None,
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        if self.path.is_none() {
            self.path = Some(path.into());
        }
        self
    }

    pub fn scheme(&self) -> &Scheme { &self.scheme }

    pub fn authority(&self) -> &str { &self.authority }

    pub fn path(&self) -> Option<&str> {
        self.path
            .as_ref()
            .map(|p| p.as_str())
    }
}
