use {
    crate::Service,
    std::collections::HashMap,
};

type ServiceMap = HashMap<String, Box<dyn Service>>;

pub(super) struct Builder(ServiceMap);

impl Builder {
    pub(super) fn new() -> Self { Self(ServiceMap::new()) }

    pub fn register(&mut self, service: impl Service + 'static) {
        let name = service
            .address()
            .authority()
            .to_string();

        self.0
            .insert(name, Box::new(service));
    }

    pub fn build(self) -> Registry { Registry(self.0) }
}

pub struct Registry(ServiceMap);

impl Registry {
    pub fn builder() -> Builder { Builder::new() }

    pub fn get(&self, address: &str) -> Option<&dyn Service> {
        self.0
            .get(address)
            .map(|s| s.as_ref())
    }

    pub fn list(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|k| k.as_str())
            .collect()
    }
}

impl From<Vec<Box<dyn Service>>> for Registry {
    fn from(services: Vec<Box<dyn Service>>) -> Self {
        let service_map = services
            .into_iter()
            .map(|service| {
                let address = service
                    .address()
                    .authority()
                    .to_string();

                (address, service)
            })
            .collect::<ServiceMap>();

        Self(service_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_empty_registry() {
        let registry = Registry::builder().build();
        assert!(registry.list().is_empty())
    }
}
