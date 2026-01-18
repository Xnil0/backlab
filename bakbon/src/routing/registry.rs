use {
    super::Service,
    crate::{
        MyErr,
        MyResult,
    },
    std::collections::HashMap,
};

type Services = Vec<Box<dyn Service>>;
type ServiceMap = HashMap<String, Services>;

#[derive(Default)]
pub struct RegistryBuilder(ServiceMap);

impl RegistryBuilder {
    pub fn register(mut self, service: impl Service + 'static) -> Self {
        let name = service.address().to_string();

        self.0
            .insert(name, vec![Box::new(service)]);

        self
    }

    pub fn build(self) -> Registry { Registry(self.0) }
}

#[derive(Default)]
pub struct Registry(pub(super) ServiceMap);

impl Registry {
    pub fn builder() -> RegistryBuilder { RegistryBuilder::default() }

    pub fn add_instance(&mut self, address: &str) -> MyResult<()> {
        let instances = self
            .0
            .get_mut(address)
            .ok_or(MyErr::ServiceNotFound)?;

        let new_instance = instances
            .last()
            .unwrap()
            .duplicate();

        instances.push(new_instance);
        Ok(())
    }

    pub fn get(&self, address: &str) -> Option<&Vec<Box<dyn Service>>> {
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
                let address = service.address().to_string();
                (address, vec![service])
            })
            .collect::<ServiceMap>();

        Self(service_map)
    }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            Address,
            Envelope,
            Reply,
        },
    };

    const ADDRESS: &str = "http://no-service.com";

    #[derive(Clone)]
    struct NoService(Address);

    impl Service for NoService {
        fn address(&self) -> &Address { &self.0 }

        fn duplicate(&self) -> Box<dyn Service> { Box::new(self.clone()) }

        fn process(&self, _message: Envelope) -> MyResult<Reply> { Ok(None) }
    }

    #[test]
    fn default_registry() {
        let registry = Registry::default();
        assert!(registry.list().is_empty());
    }

    #[test]
    fn build_registry() -> MyResult<()> {
        let address = Address::new(ADDRESS)?;
        let instance = NoService(address);
        let registry = Registry::builder()
            .register(instance)
            .build();

        let list = registry.list();
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
        assert_eq!(list[0], ADDRESS);
        Ok(())
    }

    #[test]
    fn get_instances_from_registry() -> MyResult<()> {
        let address = Address::new(ADDRESS)?;
        let instance = NoService(address);
        let registry = Registry::builder()
            .register(instance)
            .build();

        let instances = registry.get(ADDRESS);
        assert!(instances.is_some());
        assert_eq!(instances.unwrap().len(), 1);
        Ok(())
    }

    #[test]
    fn new_service_instance() -> MyResult<()> {
        let address = Address::new(ADDRESS)?;
        let instance = NoService(address);
        let mut registry = Registry::builder()
            .register(instance)
            .build();

        registry.add_instance(ADDRESS)?;
        let instances = registry.get(ADDRESS);
        assert!(instances.is_some());
        assert_eq!(instances.unwrap().len(), 2);

        Ok(())
    }
}
